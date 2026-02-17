use crate::raw::persons::RawPersonRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Person {
    pub person_id: String,
    pub person_type: Option<PersonType>,
    pub person_age: Option<u8>,
    pub person_sex: Option<PersonSex>,
    pub person_position_in_vehicle: Option<PersonPositionInVehicle>,
    pub person_role: Option<PersonRole>,
    pub crash_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonType {
    Pedestrian,
    Occupant,
    Bicyclist,
    OtherMotorized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonPositionInVehicle {
    Driver,
    Front,
    Rear,
    Lap,
    Outside,
}

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
}

impl From<RawPersonRecord> for Person {
    fn from(raw: RawPersonRecord) -> Self {
        Person {
            person_id: raw.unique_id,
            person_type: raw.person_type.as_deref().and_then(extract_person_type),
            person_age: raw.person_age,
            person_sex: raw.person_sex.and_then(extract_sex),
            person_position_in_vehicle: raw
                .person_position_in_vehicle
                .as_deref()
                .and_then(extract_position_in_vehicle),
            person_role: raw.person_ped_role.as_deref().and_then(extract_role),
            crash_id: raw.collision_id,
        }
    }
}

fn extract_person_type(raw_person_type: &str) -> Option<PersonType> {
    match raw_person_type {
        "Pedestrian" => Some(PersonType::Pedestrian),
        "Occupant" => Some(PersonType::Occupant),
        "Bicyclist" => Some(PersonType::Bicyclist),
        "Other Motorized" => Some(PersonType::OtherMotorized),
        _ => None,
    }
}

fn extract_sex(c: char) -> Option<PersonSex> {
    match c {
        'F' => Some(PersonSex::Female),
        'M' => Some(PersonSex::Male),
        _ => None,
    }
}

fn extract_position_in_vehicle(raw_position: &str) -> Option<PersonPositionInVehicle> {
    match raw_position {
        "Driver" => Some(PersonPositionInVehicle::Driver),
        "Front passenger, if two or more persons, including the driver, are in the front seat"
        | "Middle front seat, or passenger lying across a seat" => {
            Some(PersonPositionInVehicle::Front)
        }
        "Any person in the rear of a station wagon, pick-up truck, all passengers on a bus, etc"
        | "Middle rear seat, or passenger lying across a seat"
        | "Right rear passenger or motorcycle sidecar passenger"
        | "Left rear passenger, or rear passenger on a bicycle, motorcycle, snowmobile" => {
            Some(PersonPositionInVehicle::Rear)
        }
        "If one person is seated on another person&apos;s lap" => {
            Some(PersonPositionInVehicle::Lap)
        }
        "Outside" | "Riding/Hanging on Outside" => Some(PersonPositionInVehicle::Outside),
        _ => None,
    }
}

fn extract_role(raw_role: &str) -> Option<PersonRole> {
    match raw_role {
        "Notified Person" => Some(PersonRole::NotifiedPerson),
        "Witness" => Some(PersonRole::Witness),
        "Registrant" => Some(PersonRole::Registrant),
        "In-Line Skater" => Some(PersonRole::InLineSkater),
        "Passenger" => Some(PersonRole::Passenger),
        "Driver" => Some(PersonRole::Driver),
        "Policy Holder" => Some(PersonRole::PolicyHolder),
        "Owner" => Some(PersonRole::Owner),
        "Pedestrian" => Some(PersonRole::Pedestrian),
        _ => None,
    }
}
