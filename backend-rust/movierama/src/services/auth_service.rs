use crate::{
    auth::Claims,
    exceptions::MovieramaError,
    models::{AuthResponse, LoginUser, RegisterUser, User},
};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

const JWT_SECRET: &str = "JWT_SECRET";

pub async fn register_user(
    pool: &PgPool,
    data: &RegisterUser,
) -> Result<AuthResponse, MovieramaError> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?
        .to_string();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password
        "#,
        data.username,
        data.email,
        hashed,
    )
    .fetch_one(pool)
    .await?;

    let token = create_token(user, &data.password)?;

    Ok(AuthResponse { token })
}

pub async fn login_user(pool: &PgPool, data: &LoginUser) -> Result<AuthResponse, MovieramaError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password
        FROM users
        WHERE username = $1
        "#,
        data.username,
    )
    .fetch_optional(pool)
    .await?;

    let user = match user {
        Some(u) => u,
        None => return Err(MovieramaError::NotFound),
    };

    let token = create_token(user, &data.password)?;

    Ok(AuthResponse { token })
}

fn create_token(user: User, password: &str) -> Result<String, MovieramaError> {
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?;

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(MovieramaError::Unauthorized);
    }

    // Create JWT
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.username.clone(),
        user_id: user.id,
        exp: expiration,
    };

    let jwt_secret =
        std::env::var(JWT_SECRET).map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?;

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_register_user_success(pool: PgPool) {
        let data = RegisterUser {
            username: "user1".into(),
            email: "user1@mail.com".into(),
            password: "password".into(),
        };

        let result = register_user(&pool, &data).await.unwrap();
        assert!(!result.token.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_login_user_success(pool: PgPool) {
        // Register user
        register_user(
            &pool,
            &RegisterUser {
                username: "demo".into(),
                email: "demo@mail.com".into(),
                password: "password".into(),
            },
        )
        .await
        .unwrap();

        // Login
        let resp = login_user(
            &pool,
            &LoginUser {
                username: "demo".into(),
                password: "password".into(),
            },
        )
        .await
        .unwrap();

        assert!(!resp.token.is_empty());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_login_user_wrong_password(pool: PgPool) {
        register_user(
            &pool,
            &RegisterUser {
                username: "pavlos".into(),
                email: "pavlos@mail.com".into(),
                password: "password".into(),
            },
        )
        .await
        .unwrap();

        let result = login_user(
            &pool,
            &LoginUser {
                username: "pavlos".into(),
                password: "wrongpass".into(),
            },
        )
        .await;

        assert!(matches!(result, Err(MovieramaError::Unauthorized)));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_login_user_not_found(pool: PgPool) {
        let result = login_user(
            &pool,
            &LoginUser {
                username: "ghost".into(),
                password: "password".into(),
            },
        )
        .await;

        assert!(matches!(result, Err(MovieramaError::NotFound)));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_register_user_duplicate_username(pool: PgPool) {
        let data = RegisterUser {
            username: "dup".into(),
            email: "dup@mail.com".into(),
            password: "password".into(),
        };

        register_user(&pool, &data).await.unwrap();

        let duplicate = register_user(&pool, &data).await;

        assert!(matches!(duplicate, Err(MovieramaError::DatabaseError(_))));
    }
}
