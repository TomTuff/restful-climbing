use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Climber {
    pub id: Option<i32>,
    pub username: String,
}

impl Climber {
    pub fn new(id: Option<i32>, username: String) -> Self {
        Self { id, username }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NumberClimbers {
    pub number_climbers: i64,
}
