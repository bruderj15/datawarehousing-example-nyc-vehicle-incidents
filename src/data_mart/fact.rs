use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Fact {
    // dimensions
    pub contributing_factor_id: u32,
    pub person_age_id: u32,
    pub person_position_id: u32,
    pub person_role_id: u32,
    pub person_sex_id: u32,
    pub person_type_id: u32,
    pub time_id: u32,

    // measures
    pub persons_injured: u8,
    pub persons_killed: u8,
    pub pedestrians_injured: u8,
    pub pedestrians_killed: u8,
    pub cyclist_injured: u8,
    pub cyclist_killed: u8,
    pub motorist_injured: u8,
    pub motorist_killed: u8,
}
