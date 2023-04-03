use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Climb {
    pub id: Option<i32>,
    pub climber_id: i32,
    pub route_id: i32,
    #[serde(flatten)]
    pub review: Review,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rating(i32);

impl Rating {
    pub fn new(rating: i32) -> Self {
        let clamped_rating = match rating {
            i32::MIN..=0 => 1,
            1..=10 => rating,
            11..=i32::MAX => 10,
        };
        Self(clamped_rating)
    }

    pub fn i32(&self) -> i32 {
        self.0
    }
}

impl From<i32> for Rating {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub rating: Rating,
    pub review: String,
    pub completion_date: NaiveDate,
}

impl Review {
    pub fn new(rating: impl Into<Rating>, review: String, completion_date: NaiveDate) -> Self {
        Review {
            rating: rating.into(),
            review,
            completion_date,
        }
    }
}
