use crate::{
    exceptions::MovieramaError,
    models::{Movie, NewMovie},
    pagination::Pageable,
};
use chrono::Utc;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct MovieRow {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_added: chrono::DateTime<Utc>,
    pub username: String,
    pub like_count: i64,
    pub hate_count: i64,
}

pub async fn list_all_movies(
    pool: &PgPool,
    pageable: &Pageable,
) -> Result<(Vec<Movie>, u64), MovieramaError> {
    let offset = pageable.offset as i64;
    let limit = pageable.page_size as i64;
    let order_clause = pageable.sort.to_sql("m.date_added");

    let total_row = sqlx::query!("SELECT COUNT(*) as count FROM movies")
        .fetch_one(pool)
        .await?;
    let total_elements = total_row.count.unwrap_or(0) as u64;

    let query = format!(
        r#"
        SELECT
            m.id,
            m.title,
            m.description,
            m.date_added,
            u.username,
            COALESCE(SUM(CASE WHEN v.type = 'LIKE' THEN 1 ELSE 0 END), 0) AS like_count,
            COALESCE(SUM(CASE WHEN v.type = 'HATE' THEN 1 ELSE 0 END), 0) AS hate_count
        FROM movies m
        JOIN users u ON m.user_id = u.id
        LEFT JOIN votes v ON v.movie_id = m.id
        GROUP BY m.id, u.id
        ORDER BY {}
        LIMIT $1 OFFSET $2
        "#,
        order_clause
    );

    let rows = sqlx::query_as::<_, MovieRow>(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let movies = rows
        .into_iter()
        .map(|r| Movie {
            id: r.id,
            title: r.title,
            description: r.description,
            date_added: r.date_added,
            username: r.username,
            like_count: r.like_count as u64,
            hate_count: r.hate_count as u64,
        })
        .collect();

    Ok((movies, total_elements))
}

pub async fn list_all_movies_by_username(
    pool: &PgPool,
    pageable: &Pageable,
    username: &str,
) -> Result<(Vec<Movie>, u64), MovieramaError> {
    let offset = pageable.offset as i64;
    let limit = pageable.page_size as i64;
    let order_clause = pageable.sort.to_sql("m.date_added");

    let total_row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM movies m
        JOIN users u ON m.user_id = u.id
        WHERE u.username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?;
    let total_elements = total_row.count.unwrap_or(0) as u64;

    let query = format!(
        r#"
        SELECT
            m.id,
            m.title,
            m.description,
            m.date_added,
            u.username,
            COALESCE(SUM(CASE WHEN v.type = 'LIKE' THEN 1 ELSE 0 END), 0) AS like_count,
            COALESCE(SUM(CASE WHEN v.type = 'HATE' THEN 1 ELSE 0 END), 0) AS hate_count
        FROM movies m
        JOIN users u ON m.user_id = u.id
        LEFT JOIN votes v ON v.movie_id = m.id
        WHERE username = $3
        GROUP BY m.id, u.id
        ORDER BY {}
        LIMIT $1 OFFSET $2
        "#,
        order_clause
    );

    let rows = sqlx::query_as::<_, MovieRow>(&query)
        .bind(limit)
        .bind(offset)
        .bind(username)
        .fetch_all(pool)
        .await?;

    let movies = rows
        .into_iter()
        .map(|r| Movie {
            id: r.id,
            title: r.title,
            description: r.description,
            date_added: r.date_added,
            username: r.username,
            like_count: r.like_count as u64,
            hate_count: r.hate_count as u64,
        })
        .collect();

    Ok((movies, total_elements))
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
        username: m.username,
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
        username: rec.username,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewMovie, RegisterUser};
    use crate::pagination::{Pageable, Sort};
    use crate::services::auth_service;
    use sqlx::PgPool;

    fn create_pagination(page: u32, size: u32, sort: &str) -> Pageable {
        Pageable::new(page, size, Sort::from_query(sort))
    }

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

    async fn create_test_movie(pool: &PgPool, user_id: i32, title: &str) -> Movie {
        let new_movie = NewMovie {
            title: title.into(),
            description: Some(format!("Description for {}", title)),
        };
        create_movie(pool, user_id, new_movie).await.unwrap()
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_movie(pool: PgPool) {
        let user_id = create_user(&pool, "movie_creator").await;

        let new_movie = NewMovie {
            title: "Test Movie".into(),
            description: Some("A great test movie".into()),
        };

        let result = create_movie(&pool, user_id, new_movie).await.unwrap();

        assert_eq!(result.title, "Test Movie");
        assert_eq!(result.description, Some("A great test movie".into()));
        assert_eq!(result.like_count, 0);
        assert_eq!(result.hate_count, 0);
        assert!(!result.username.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_movie_by_id(pool: PgPool) {
        let user_id = create_user(&pool, "movie_getter").await;
        let movie = create_test_movie(&pool, user_id, "Get Me").await;

        let result = get_movie_by_id(&pool, movie.id).await.unwrap();

        assert!(result.is_some());
        let movie = result.unwrap();
        assert_eq!(movie.title, "Get Me");
        assert_eq!(movie.like_count, 0);
        assert_eq!(movie.hate_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_movie_by_id_not_found(pool: PgPool) {
        let result = get_movie_by_id(&pool, 99999).await.unwrap();

        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_all_movies(pool: PgPool) {
        let user1_id = create_user(&pool, "user1").await;
        let user2_id = create_user(&pool, "user2").await;

        create_test_movie(&pool, user1_id, "Movie One").await;
        create_test_movie(&pool, user2_id, "Movie Two").await;

        let pageable = create_pagination(0, 10, "dateAdded,desc");

        let (movies, total) = list_all_movies(&pool, &pageable).await.unwrap();

        assert_eq!(total, 2);
        assert_eq!(movies.len(), 2);

        let titles: Vec<String> = movies.iter().map(|m| m.title.clone()).collect();
        assert!(titles.contains(&"Movie One".into()));
        assert!(titles.contains(&"Movie Two".into()));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_all_movies_pagination(pool: PgPool) {
        let user_id = create_user(&pool, "pagination_user").await;

        // Create 3 movies
        create_test_movie(&pool, user_id, "Movie 1").await;
        create_test_movie(&pool, user_id, "Movie 2").await;
        create_test_movie(&pool, user_id, "Movie 3").await;

        // Test first page with 2 items
        let pageable = create_pagination(0, 2, "dateAdded,desc");
        let (movies, total) = list_all_movies(&pool, &pageable).await.unwrap();

        assert_eq!(total, 3);
        assert_eq!(movies.len(), 2);

        // Test second page with 2 items
        let pageable = create_pagination(1, 2, "dateAdded,desc");
        let (movies, total) = list_all_movies(&pool, &pageable).await.unwrap();

        assert_eq!(total, 3);
        assert_eq!(movies.len(), 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_all_movies_empty_page(pool: PgPool) {
        let user_id = create_user(&pool, "empty_page_user").await;
        create_test_movie(&pool, user_id, "Single Movie").await;

        // Request page that doesn't exist
        let pageable = create_pagination(5, 10, "dateAdded,desc");
        let (movies, total) = list_all_movies(&pool, &pageable).await.unwrap();

        assert_eq!(total, 1);
        assert_eq!(movies.len(), 0); // Empty result for out-of-bounds page
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_all_movies_by_username(pool: PgPool) {
        let user1_id = create_user(&pool, "specific_user").await;
        let user2_id = create_user(&pool, "other_user").await;

        create_test_movie(&pool, user1_id, "User1 Movie").await;
        create_test_movie(&pool, user2_id, "User2 Movie").await;
        create_test_movie(&pool, user1_id, "Another User1 Movie").await;

        let pageable = create_pagination(0, 10, "dateAdded,desc");

        let (movies, total) = list_all_movies_by_username(&pool, &pageable, "specific_user")
            .await
            .unwrap();

        assert_eq!(total, 2);
        assert_eq!(movies.len(), 2);

        for movie in movies {
            assert_eq!(movie.username, "specific_user");
            assert!(movie.title.contains("User1"));
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_all_movies_by_username_not_found(pool: PgPool) {
        let pageable = create_pagination(0, 10, "dateAdded,desc");

        let (movies, total) = list_all_movies_by_username(&pool, &pageable, "nonexistent_user")
            .await
            .unwrap();

        assert_eq!(total, 0);
        assert_eq!(movies.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_all_movies_by_username_pagination(pool: PgPool) {
        let user_id = create_user(&pool, "paged_user").await;

        // Create 5 movies for this user
        for i in 1..=5 {
            create_test_movie(&pool, user_id, &format!("Movie {}", i)).await;
        }

        // First page - 2 movies
        let pageable = create_pagination(0, 2, "dateAdded,desc");
        let (movies, total) = list_all_movies_by_username(&pool, &pageable, "paged_user")
            .await
            .unwrap();

        assert_eq!(total, 5);
        assert_eq!(movies.len(), 2);

        // Second page - 2 movies
        let pageable = create_pagination(1, 2, "dateAdded,desc");
        let (movies, total) = list_all_movies_by_username(&pool, &pageable, "paged_user")
            .await
            .unwrap();

        assert_eq!(total, 5);
        assert_eq!(movies.len(), 2);

        // Third page - 1 movie
        let pageable = create_pagination(2, 2, "dateAdded,desc");
        let (movies, total) = list_all_movies_by_username(&pool, &pageable, "paged_user")
            .await
            .unwrap();

        assert_eq!(total, 5);
        assert_eq!(movies.len(), 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_movie(pool: PgPool) {
        let user_id = create_user(&pool, "deleter").await;
        let movie = create_test_movie(&pool, user_id, "To Delete").await;

        let deleted = delete_movie(&pool, movie.id).await.unwrap();
        assert!(deleted);

        // Verify movie is gone
        let result = get_movie_by_id(&pool, movie.id).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_movie_not_found(pool: PgPool) {
        let deleted = delete_movie(&pool, 99999).await.unwrap();
        assert!(!deleted);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_movie(pool: PgPool) {
        let user_id = create_user(&pool, "updater").await;
        let movie = create_test_movie(&pool, user_id, "Original Title").await;

        let update_data = NewMovie {
            title: "Updated Title".into(),
            description: Some("Updated description".into()),
        };

        let result = update_movie(&pool, movie.id, update_data).await.unwrap();

        assert_eq!(result.title, "Updated Title");
        assert_eq!(result.description, Some("Updated description".into()));

        // Verify the update persisted
        let updated_movie = get_movie_by_id(&pool, movie.id).await.unwrap().unwrap();
        assert_eq!(updated_movie.title, "Updated Title");
        assert_eq!(
            updated_movie.description,
            Some("Updated description".into())
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_movie_not_found(pool: PgPool) {
        let update_data = NewMovie {
            title: "New Title".into(),
            description: Some("New description".into()),
        };

        let result = update_movie(&pool, 99999, update_data).await;

        assert!(matches!(result, Err(MovieramaError::NotFound)));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_movie_counts_with_votes(pool: PgPool) {
        let user1_id = create_user(&pool, "vote_owner").await;
        let user2_id = create_user(&pool, "voter1").await;
        let user3_id = create_user(&pool, "voter2").await;

        let movie = create_test_movie(&pool, user1_id, "Voted Movie").await;

        // Add some votes
        use crate::services::vote_service;
        vote_service::insert_vote(&pool, user2_id, movie.id, crate::models::VoteType::Like)
            .await
            .unwrap();
        vote_service::insert_vote(&pool, user3_id, movie.id, crate::models::VoteType::Like)
            .await
            .unwrap();

        let result = get_movie_by_id(&pool, movie.id).await.unwrap().unwrap();

        assert_eq!(result.like_count, 2);
        assert_eq!(result.hate_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_list_movies_sorting(pool: PgPool) {
        let user_id = create_user(&pool, "sorter").await;

        // Create movies with different dates
        let movie1 = create_test_movie(&pool, user_id, "First Movie").await;
        let movie2 = create_test_movie(&pool, user_id, "Second Movie").await;

        // Test descending order (newest first)
        let pageable_desc = create_pagination(0, 10, "dateAdded,desc");
        let (movies_desc, _) = list_all_movies(&pool, &pageable_desc).await.unwrap();
        assert_eq!(movies_desc[0].id, movie2.id); // Second movie should be first (newer)
        assert_eq!(movies_desc[1].id, movie1.id);

        // Test ascending order (oldest first)
        let pageable_asc = create_pagination(0, 10, "dateAdded,asc");
        let (movies_asc, _) = list_all_movies(&pool, &pageable_asc).await.unwrap();
        assert_eq!(movies_asc[0].id, movie1.id); // First movie should be first (older)
        assert_eq!(movies_asc[1].id, movie2.id);
    }
}
