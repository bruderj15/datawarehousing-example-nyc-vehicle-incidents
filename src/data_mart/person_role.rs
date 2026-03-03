use serde::{Deserialize, Serialize};
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumCountMacro)]
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

impl PersonPositionRole {
    pub fn gen_positions_roles() -> Vec<PersonPositionRole> {
        vec![
            PersonPositionRole {
                person_position_role_id: 0,
                person_position_role: PersonRole::Unknown,
            },
            PersonPositionRole {
                person_position_role_id: 1,
                person_position_role: PersonRole::NotifiedPerson,
            },
            PersonPositionRole {
                person_position_role_id: 2,
                person_position_role: PersonRole::Witness,
            },
            PersonPositionRole {
                person_position_role_id: 3,
                person_position_role: PersonRole::Registrant,
            },
            PersonPositionRole {
                person_position_role_id: 4,
                person_position_role: PersonRole::InLineSkater,
            },
            PersonPositionRole {
                person_position_role_id: 5,
                person_position_role: PersonRole::Passenger,
            },
            PersonPositionRole {
                person_position_role_id: 6,
                person_position_role: PersonRole::Driver,
            },
            PersonPositionRole {
                person_position_role_id: 7,
                person_position_role: PersonRole::PolicyHolder,
            },
            PersonPositionRole {
                person_position_role_id: 8,
                person_position_role: PersonRole::Owner,
            },
            PersonPositionRole {
                person_position_role_id: 9,
                person_position_role: PersonRole::Pedestrian,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::EnumCount;

    #[test]
    fn gen_positions_roles_length_matches_enum_count() {
        assert_eq!(
            PersonPositionRole::gen_positions_roles().len(),
            PersonRole::COUNT,
        );
    }
}
