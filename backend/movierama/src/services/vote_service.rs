use std::collections::HashMap;

use sqlx::{FromRow, PgPool};

use crate::{
    exceptions::MovieramaError,
    models::{Movie, VoteType},
    services::movie_service,
};

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct VoteRow {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub vote_type: String,
}

pub async fn vote_movie(
    pool: &PgPool,
    user_id: i32,
    movie_id: i32,
    vote_type: VoteType,
) -> Result<Movie, MovieramaError> {
    match get_vote(pool, user_id, movie_id).await? {
        Some(vtype) => {
            if vtype == vote_type {
                // Retract vote
                delete_vote(pool, user_id, movie_id).await?;
            } else {
                // Reverse vote
                update_vote(pool, user_id, movie_id, vote_type).await?;
            }
        }
        // Simple vote
        None => {
            insert_vote(pool, user_id, movie_id, vote_type).await?;
        }
    };

    match movie_service::get_movie_by_id(pool, movie_id).await? {
        Some(mov) => Ok(mov),
        None => Err(MovieramaError::NotFound),
    }
}

pub async fn get_vote(
    pool: &PgPool,
    user_id: i32,
    movie_id: i32,
) -> Result<Option<VoteType>, MovieramaError> {
    let vote_row = sqlx::query_as!(
        VoteRow,
        r#"
         SELECT id, user_id, movie_id, type as vote_type
         FROM votes  
         WHERE user_id = $1
         AND movie_id = $2
       "#,
        user_id,
        movie_id,
    )
    .fetch_optional(pool)
    .await?;

    if vote_row.is_none() {
        return Ok(None);
    }

    Ok(Some(vote_row.unwrap().vote_type.parse()?))
}

pub async fn delete_vote(pool: &PgPool, user_id: i32, movie_id: i32) -> Result<(), MovieramaError> {
    sqlx::query!(
        r#"
        DELETE FROM votes
        WHERE user_id = $1 AND movie_id = $2  
        "#,
        user_id,
        movie_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_vote(
    pool: &PgPool,
    user_id: i32,
    movie_id: i32,
    vote_type: VoteType,
) -> Result<(), MovieramaError> {
    sqlx::query!(
        r#"
        INSERT INTO votes (movie_id, user_id, type) 
        VALUES ($1, $2, $3)
        "#,
        movie_id,
        user_id,
        vote_type as VoteType,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_vote(
    pool: &PgPool,
    user_id: i32,
    movie_id: i32,
    vote_type: VoteType,
) -> Result<(), MovieramaError> {
    sqlx::query!(
        r#"
        UPDATE votes SET type = $3
        WHERE user_id = $1 AND movie_id = $2
        "#,
        user_id,
        movie_id,
        vote_type as VoteType,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_votes_for_movies(
    pool: &PgPool,
    user_id: i32,
    movie_ids: &[i32],
) -> Result<HashMap<i32, VoteType>, MovieramaError> {
    if movie_ids.is_empty() {
        return Ok(HashMap::new());
    }

    // Create placeholders for the IN clause
    let placeholders: Vec<String> = (1..=movie_ids.len()).map(|i| format!("${}", i)).collect();
    let placeholders_str = placeholders.join(", ");

    let query = format!(
        r#"
        SELECT movie_id, type
        FROM votes
        WHERE user_id = $1 AND movie_id IN ({})
        "#,
        placeholders_str
    );

    let mut query = sqlx::query_as::<_, (i32, String)>(&query).bind(user_id);

    // Bind each movie_id parameter
    for movie_id in movie_ids {
        query = query.bind(movie_id);
    }

    let rows = query.fetch_all(pool).await?;

    let votes_map = rows
        .into_iter()
        .filter_map(|(movie_id, vote_type_str)| {
            vote_type_str
                .parse::<VoteType>()
                .ok()
                .map(|vote_type| (movie_id, vote_type))
        })
        .collect();

    Ok(votes_map)
}
