use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonPositionInVehicle {
    Driver,
    Front,
    Rear,
    Lap,
    Outside,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonPosition {
    pub person_position_id: u32,
    pub person_position: PersonPositionInVehicle,
}
