use thiserror::Error;
use std::fmt::Display;

#[derive(Error, Debug)]
pub enum DatabaseError {
    FailedConnection(#[from] sqlx::Error),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Self::FailedConnection(_) => { "Failed to connect to the database" }
        };
        write!(f, "{}", display_str)
    }
}