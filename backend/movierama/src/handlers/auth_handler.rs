use crate::models::{NewUser, User};
use axum::Json;

pub async fn signup(Json(payload): Json<NewUser>) -> Json<User> {
    let user = User {
        id: 1,
        username: payload.username,
        email: payload.email,
        password: payload.password,
    };
    Json(user)
}

pub async fn login(Json(payload): Json<NewUser>) -> Json<String> {
    Json(format!("Welcome, {}! (dummy token)", payload.username))
}
