use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonAgeGroup {
    Fertile,
    Infertile,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonAge {
    pub person_age_id: u32,
    pub person_age: u8,
    pub person_age_hier_def_group: PersonAgeGroup,
}
