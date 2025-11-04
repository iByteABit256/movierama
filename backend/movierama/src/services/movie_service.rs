use crate::{
    exceptions::MovieramaError,
    models::{Movie, NewMovie, UserSummary},
};
use chrono::Utc;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct MovieRow {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_added: chrono::DateTime<Utc>,
    pub user_id: i32,
    pub username: String,
    pub like_count: i64,
    pub hate_count: i64,
}

pub async fn list_all_movies(pool: &PgPool) -> Result<Vec<Movie>, MovieramaError> {
    let rows = sqlx::query_as!(
        MovieRow,
        r#"
        SELECT
            m.id,
            m.title,
            m.description,
            m.date_added,
            u.id AS user_id,
            u.username,
            COALESCE(SUM(CASE WHEN v.type = 'LIKE' THEN 1 ELSE 0 END), 0) AS "like_count!: i64",
            COALESCE(SUM(CASE WHEN v.type = 'HATE' THEN 1 ELSE 0 END), 0) AS "hate_count!: i64"
        FROM movies m
        JOIN users u ON m.user_id = u.id
        LEFT JOIN votes v ON v.movie_id = m.id
        GROUP BY m.id, u.id
        ORDER BY m.date_added DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Movie {
            id: r.id,
            title: r.title,
            description: r.description,
            date_added: r.date_added,
            user: UserSummary {
                id: r.user_id,
                username: r.username,
            },
            like_count: r.like_count as u64,
            hate_count: r.hate_count as u64,
        })
        .collect())
}

pub async fn get_movie_by_id(
    pool: &PgPool,
    movie_id: i32,
) -> Result<Option<Movie>, MovieramaError> {
    let movie = sqlx::query_as!(
        MovieRow,
        r#"
        SELECT
            m.id,
            m.title,
            m.description,
            m.date_added,
            u.id AS user_id,
            u.username,
            COALESCE(SUM(CASE WHEN v.type = 'LIKE' THEN 1 ELSE 0 END), 0) AS "like_count!: i64",
            COALESCE(SUM(CASE WHEN v.type = 'HATE' THEN 1 ELSE 0 END), 0) AS "hate_count!: i64"
        FROM movies m
        JOIN users u ON m.user_id = u.id
        LEFT JOIN votes v ON v.movie_id = m.id
        WHERE m.id = $1
        GROUP BY m.id, u.id
        "#,
        movie_id,
    )
    .fetch_optional(pool)
    .await?
    .map(|m| Movie {
        id: m.id,
        title: m.title,
        description: m.description,
        date_added: m.date_added,
        user: UserSummary {
            id: m.user_id,
            username: m.username,
        },
        like_count: m.like_count as u64,
        hate_count: m.hate_count as u64,
    });

    Ok(movie)
}

pub async fn delete_movie(pool: &PgPool, movie_id: i32) -> Result<bool, MovieramaError> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE
        FROM movies m
        WHERE m.id = $1
        "#,
        movie_id,
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn create_movie(
    pool: &PgPool,
    user_id: i32,
    data: NewMovie,
) -> Result<Movie, MovieramaError> {
    let rec = sqlx::query_as!(
        MovieRow,
        r#"
        INSERT INTO movies (title, description, user_id)
        VALUES ($1, $2, $3)
        RETURNING 
            id,
            title,
            description,
            date_added,
            $3 AS "user_id!: i32",
            (SELECT username FROM users WHERE id = $3) AS "username!: String",
            0 AS "like_count!: i64",
            0 AS "hate_count!: i64"
        "#,
        data.title,
        data.description,
        user_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(Movie {
        id: rec.id,
        title: rec.title,
        description: rec.description,
        date_added: rec.date_added,
        user: UserSummary {
            id: rec.user_id,
            username: rec.username,
        },
        like_count: 0,
        hate_count: 0,
    })
}

pub async fn update_movie(
    pool: &PgPool,
    movie_id: i32,
    data: NewMovie,
) -> Result<NewMovie, MovieramaError> {
    let result = sqlx::query_as!(
        NewMovie,
        r#"
        UPDATE movies
        SET title = $1, description = $2
        WHERE id = $3
        RETURNING 
            title,
            description
        "#,
        data.title,
        data.description,
        movie_id,
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(updated_movie) => Ok(updated_movie),
        Err(sqlx::Error::RowNotFound) => Err(MovieramaError::NotFound),
        Err(e) => Err(MovieramaError::DatabaseError(e)),
    }
}
