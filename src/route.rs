/// module for structs related to Route, mostly handled by the API endpoint /routes
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DifficultyRating(u8);

impl Into<DifficultyRating> for usize {
    fn into(self) -> DifficultyRating {
        let rating = match self {
            0 => 1,
            1..=10 => self as u8,
            _ => 10,
        };
        DifficultyRating(rating)
    }
}

// There are some crates that do some actual work with GPS coordinates, but I want to narrow the
// scope of the project here, and just assume any pair of f64 is a valid GPS position
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GpsPosition(f64, f64);

impl Into<GpsPosition> for (f64, f64) {
    fn into(self) -> GpsPosition {
        GpsPosition(self.0, self.1)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Route {
    name: String,
    difficulty: DifficultyRating,
    location: GpsPosition,
}

impl Route {
    pub fn new(
        name: String,
        difficulty: impl Into<DifficultyRating>,
        location: impl Into<GpsPosition>,
    ) -> Self {
        Self {
            name,
            difficulty: difficulty.into(),
            location: location.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_route() -> Route {
        Route::new("funky monkey".to_string(), 5, (123.45, 52.310))
    }

    #[test]
    fn test_dummy_route() {
        let dummmy_route = test_route();
        assert_eq!(
            dummmy_route,
            Route {
                name: "funky monkey".to_string(),
                difficulty: DifficultyRating(5),
                location: GpsPosition(123.45, 52.310),
            }
        )
    }
}
