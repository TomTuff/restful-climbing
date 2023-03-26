use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    FailedConnection(#[from] sqlx::Error),
    FailedParseDifficultyRating,
    UpdateRouteAllNone,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Self::FailedConnection(_) => "Failed to connect to the database",
            Self::FailedParseDifficultyRating => {
                "Failed to parse a difficulty rating from the database"
            }
            Self::UpdateRouteAllNone => {
                "All fields were None when trying to update a route in the database"
            }
        };
        write!(f, "{}", display_str)
    }
}
