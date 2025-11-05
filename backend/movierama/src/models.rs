use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::exceptions::MovieramaError;

//
// ===== Enums =====
//

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum VoteType {
    Like,
    Hate,
}

impl FromStr for VoteType {
    type Err = MovieramaError;

    fn from_str(input: &str) -> Result<VoteType, Self::Err> {
        match input {
            "LIKE" => Ok(VoteType::Like),
            "HATE" => Ok(VoteType::Hate),
            _ => Err(MovieramaError::BadRequest(
                "Invalid vote type, available options are 'LIKE' and 'HATE'.".to_owned(),
            )),
        }
    }
}

//
// ===== Core Models =====
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // don’t expose password in API responses
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "dateAdded")]
    pub date_added: DateTime<Utc>,
    pub user: UserSummary,
    #[serde(rename = "likeCount")]
    pub like_count: u64,
    #[serde(rename = "hateCount")]
    pub hate_count: u64,
}

//
// ===== Summary / DTO types =====
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSummary {
    pub id: i32,
    pub username: String,
}

//
// ===== DTOs for creation =====
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewMovie {
    pub title: String,
    pub description: Option<String>,
}
