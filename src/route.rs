use serde::{Deserialize, Serialize};
/// module for structs related to Route, mostly handled by the API endpoint /routes
use std::fmt::Display;
use std::str::FromStr;

use crate::error::DatabaseError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DifficultyRating {
    Rating59,
    Rating510,
    Rating511,
    Rating511plus,
    Rating512,
}

impl Display for DifficultyRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Self::Rating59 => "5.9",
            Self::Rating510 => "5.10",
            Self::Rating511 => "5.11",
            Self::Rating511plus => "5.11+",
            Self::Rating512 => "5.12",
        };
        write!(f, "{}", display_str)
    }
}

impl FromStr for DifficultyRating {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "5.9" => Ok(Self::Rating59),
            "5.10" => Ok(Self::Rating510),
            "5.11" => Ok(Self::Rating511),
            "5.11+" => Ok(Self::Rating511plus),
            "5.12" => Ok(Self::Rating512),
            _ => Err(DatabaseError::FailedParseDifficultyRating),
        }
    }
}

// There are some crates that do some actual work with GPS coordinates, but I want to narrow the
// scope of the project here, and just assume any pair of f64 is a valid GPS position
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GpsPosition(f64, f64);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Route {
    pub name: String,
    pub difficulty: DifficultyRating,
    pub latitude: f64,
    pub longitude: f64,
}

impl Route {
    pub fn new(
        name: String,
        difficulty: impl Into<DifficultyRating>,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            name,
            difficulty: difficulty.into(),
            latitude,
            longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_route() -> Route {
        Route::new(
            "funky monkey".to_string(),
            DifficultyRating::Rating59,
            123.45,
            52.310,
        )
    }

    #[test]
    fn test_dummy_route() {
        let dummmy_route = test_route();
        assert_eq!(
            dummmy_route,
            Route {
                name: "funky monkey".to_string(),
                difficulty: DifficultyRating::Rating59,
                latitude: 123.45,
                longitude: 52.310,
            }
        )
    }
}
