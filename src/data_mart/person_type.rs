use serde::{Deserialize, Serialize};
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumCountMacro)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonTypeType {
    Pedestrian,
    Occupant,
    Bicyclist,
    OtherMotorized,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonType {
    pub person_type_id: u32,
    pub person_type: PersonTypeType,
}

impl PersonType {
    pub fn gen_types() -> Vec<PersonType> {
        vec![
            PersonType {
                person_type_id: 0,
                person_type: PersonTypeType::Unknown,
            },
            PersonType {
                person_type_id: 1,
                person_type: PersonTypeType::Pedestrian,
            },
            PersonType {
                person_type_id: 2,
                person_type: PersonTypeType::Occupant,
            },
            PersonType {
                person_type_id: 3,
                person_type: PersonTypeType::Bicyclist,
            },
            PersonType {
                person_type_id: 4,
                person_type: PersonTypeType::OtherMotorized,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::EnumCount;

    #[test]
    fn gen_types_length_matches_enum_count() {
        assert_eq!(PersonType::gen_types().len(), PersonTypeType::COUNT);
    }
}
