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
    pub person_age_known: bool,
    pub person_age_hier_def_group: PersonAgeGroup,
}

impl PersonAge {
    pub fn gen_ages() -> Vec<PersonAge> {
        let mut ages = (1..=121)
            .map(|age| PersonAge {
                person_age_id: age as u32,
                person_age: age,
                person_age_known: true,
                person_age_hier_def_group: if (15..=49).contains(&age) {
                    PersonAgeGroup::Fertile
                } else {
                    PersonAgeGroup::Infertile
                },
            })
            .collect::<Vec<_>>();
        ages.push(PersonAge {
            person_age_id: 0,
            person_age: 0,
            person_age_known: false,
            person_age_hier_def_group: PersonAgeGroup::Unknown,
        });
        ages
    }
}
