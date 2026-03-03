use serde::{Deserialize, Serialize};
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumCountMacro)]
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

impl PersonPosition {
    pub fn gen_positions() -> Vec<PersonPosition> {
        vec![
            PersonPosition {
                person_position_id: 0,
                person_position: PersonPositionInVehicle::Unknown,
            },
            PersonPosition {
                person_position_id: 1,
                person_position: PersonPositionInVehicle::Driver,
            },
            PersonPosition {
                person_position_id: 2,
                person_position: PersonPositionInVehicle::Front,
            },
            PersonPosition {
                person_position_id: 3,
                person_position: PersonPositionInVehicle::Rear,
            },
            PersonPosition {
                person_position_id: 4,
                person_position: PersonPositionInVehicle::Lap,
            },
            PersonPosition {
                person_position_id: 5,
                person_position: PersonPositionInVehicle::Outside,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::EnumCount;

    #[test]
    fn gen_positions_length_matches_enum_count() {
        assert_eq!(
            PersonPosition::gen_positions().len(),
            PersonPositionInVehicle::COUNT,
        );
    }
}
