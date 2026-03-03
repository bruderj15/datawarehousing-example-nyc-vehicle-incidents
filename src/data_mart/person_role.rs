use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonRole {
    NotifiedPerson,
    Witness,
    Registrant,
    InLineSkater,
    Passenger,
    Driver,
    PolicyHolder,
    Owner,
    Pedestrian,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonPositionRole {
    pub person_position_role_id: u32,
    pub person_position_role: PersonRole,
}
