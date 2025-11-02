use crate::models::{Vote, VoteType};

pub async fn create_vote(movie_id: i32, user_id: i32, vote_type: VoteType) -> Vote {
    Vote {
        id: Some(99),
        movie_id,
        user_id,
        vote_type,
    }
}

pub async fn remove_vote(movie_id: i32, user_id: i32) -> bool {
    println!("Removing vote for user {} on movie {}", user_id, movie_id);
    true
}
