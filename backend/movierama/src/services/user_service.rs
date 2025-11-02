use crate::models::{Movie, NewUser, User, UserSummary};
use chrono::Utc;

/// Register a new user
pub async fn create_user(data: NewUser) -> User {
    User {
        id: 1,
        username: data.username,
        email: data.email,
        password: data.password,
    }
}

/// Fetch all movies created by a specific user
pub async fn list_movies_by_user(user_id: i32) -> Vec<Movie> {
    let user = UserSummary {
        id: user_id,
        username: format!("user_{}", user_id),
    };

    vec![
        Movie {
            id: 1,
            title: "User's First Movie".into(),
            description: Some("Description here".into()),
            date_added: Utc::now(),
            user: user.clone(),
            like_count: 3,
            hate_count: 1,
        },
        Movie {
            id: 2,
            title: "User's Second Movie".into(),
            description: Some("Another movie".into()),
            date_added: Utc::now(),
            user,
            like_count: 1,
            hate_count: 0,
        },
    ]
}
