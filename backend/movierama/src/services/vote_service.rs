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
    if movie_service::get_movie_by_id(pool, movie_id)
        .await?
        .is_none()
    {
        return Err(MovieramaError::NotFound);
    }

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
    }

    Ok(movie_service::get_movie_by_id(pool, movie_id)
        .await?
        .unwrap())
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
    let placeholders: Vec<String> = (2..=movie_ids.len() + 1)
        .map(|i| format!("${}", i))
        .collect();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewMovie, RegisterUser};
    use crate::services::{auth_service, movie_service};
    use sqlx::PgPool;

    async fn create_user(pool: &PgPool, username: &str) -> i32 {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret");
        }

        let reg = RegisterUser {
            username: username.into(),
            email: format!("{}@mail.com", username),
            password: "password".into(),
        };
        let auth = auth_service::register_user(pool, &reg).await.unwrap();

        let claims = jsonwebtoken::decode::<crate::auth::Claims>(
            &auth.token,
            &jsonwebtoken::DecodingKey::from_secret("test-secret".as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .unwrap()
        .claims;

        claims.user_id
    }

    async fn create_movie(pool: &PgPool, user_id: i32, title: &str) -> i32 {
        let movie = movie_service::create_movie(
            pool,
            user_id,
            NewMovie {
                title: title.into(),
                description: Some("desc".into()),
            },
        )
        .await
        .unwrap();

        movie.id
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_vote(pool: PgPool) {
        let uid = create_user(&pool, "voter").await;
        let mid = create_movie(&pool, uid, "movie1").await;

        // Add LIKE vote
        let result = vote_movie(&pool, uid, mid, VoteType::Like).await.unwrap();

        assert_eq!(result.like_count, 1);
        assert_eq!(result.hate_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_reverse_vote(pool: PgPool) {
        let uid = create_user(&pool, "revuser").await;
        let mid = create_movie(&pool, uid, "movie2").await;

        // First LIKE
        vote_movie(&pool, uid, mid, VoteType::Like).await.unwrap();

        // Then switch to HATE
        let updated = vote_movie(&pool, uid, mid, VoteType::Hate).await.unwrap();

        assert_eq!(updated.like_count, 0);
        assert_eq!(updated.hate_count, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_retract_vote(pool: PgPool) {
        let uid = create_user(&pool, "retruser").await;
        let mid = create_movie(&pool, uid, "movie3").await;

        // First LIKE
        vote_movie(&pool, uid, mid, VoteType::Like).await.unwrap();

        // Like again → retract (remove vote)
        let updated = vote_movie(&pool, uid, mid, VoteType::Like).await.unwrap();

        assert_eq!(updated.like_count, 0);
        assert_eq!(updated.hate_count, 0);

        // Ensure no vote exists
        let v = get_vote(&pool, uid, mid).await.unwrap();
        assert!(v.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_vote(pool: PgPool) {
        let uid = create_user(&pool, "getv").await;
        let mid = create_movie(&pool, uid, "movie4").await;

        insert_vote(&pool, uid, mid, VoteType::Hate).await.unwrap();

        let vote = get_vote(&pool, uid, mid).await.unwrap();

        assert_eq!(vote, Some(VoteType::Hate));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_user_votes_for_movies(pool: PgPool) {
        let uid = create_user(&pool, "batch").await;

        let m1 = create_movie(&pool, uid, "m1").await;
        let m2 = create_movie(&pool, uid, "m2").await;
        let m3 = create_movie(&pool, uid, "m3").await;

        // Votes:
        insert_vote(&pool, uid, m1, VoteType::Like).await.unwrap();
        insert_vote(&pool, uid, m3, VoteType::Hate).await.unwrap();

        let results = get_user_votes_for_movies(&pool, uid, &[m1, m2, m3])
            .await
            .unwrap();

        assert_eq!(results.get(&m1), Some(&VoteType::Like));
        assert_eq!(results.get(&m2), None);
        assert_eq!(results.get(&m3), Some(&VoteType::Hate));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_vote_movie_not_found(pool: PgPool) {
        let uid = create_user(&pool, "nofound").await;

        let result = vote_movie(&pool, uid, 99999, VoteType::Like).await;

        assert!(matches!(result, Err(MovieramaError::NotFound)));
    }
}
