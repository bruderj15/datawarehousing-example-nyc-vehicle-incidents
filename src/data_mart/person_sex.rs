use serde::{Deserialize, Serialize};
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumCountMacro)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonSexType {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonSex {
    pub person_sex_id: u32,
    pub person_sex: PersonSexType,
}

impl PersonSex {
    pub fn gen_sexes() -> Vec<PersonSex> {
        vec![
            PersonSex {
                person_sex_id: 0,
                person_sex: PersonSexType::Unknown,
            },
            PersonSex {
                person_sex_id: 1,
                person_sex: PersonSexType::Male,
            },
            PersonSex {
                person_sex_id: 2,
                person_sex: PersonSexType::Female,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::EnumCount;

    #[test]
    fn gen_sexes_length_matches_enum_count() {
        assert_eq!(PersonSex::gen_sexes().len(), PersonSexType::COUNT);
    }
}
