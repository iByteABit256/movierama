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

pub async fn register_user(pool: &PgPool, data: RegisterUser) -> Result<User, MovieramaError> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?
        .to_string();

    let rec = sqlx::query_as!(
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

    Ok(rec)
}

pub async fn login_user(pool: &PgPool, data: LoginUser) -> Result<AuthResponse, MovieramaError> {
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

    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|e| MovieramaError::UnexpectedError(e.to_string()))?;

    if Argon2::default()
        .verify_password(data.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(MovieramaError::UnexpectedError(
            "Invalid credentials".into(),
        ));
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

    Ok(AuthResponse { token })
}
