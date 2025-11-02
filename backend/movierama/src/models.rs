use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//
// ===== Enums =====
//

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum VoteType {
    Like,
    Hate,
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
    // Optional list of movies; useful for nested serialization later
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub movies: Option<Vec<MovieSummary>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_added: DateTime<Utc>,
    pub user: UserSummary,
    pub like_count: u64,
    pub hate_count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote {
    pub id: Option<i32>,
    pub movie_id: i32,
    pub user_id: i32,
    pub vote_type: VoteType,
}

//
// ===== Summary / DTO types =====
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSummary {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieSummary {
    pub id: i32,
    pub title: String,
}

//
// ===== DTOs for creation =====
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewMovie {
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32, // To be removed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewVote {
    pub movie_id: i64,
    pub vote_type: VoteType,
}
