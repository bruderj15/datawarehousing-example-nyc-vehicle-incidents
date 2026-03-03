use crate::base_database::{crash::Crash, person::Person};
use crate::data_mart::{
    contributing_factor::{ContributingFactor, ContributingFactorDim},
    person_age::PersonAge,
    person_position::{PersonPosition, PersonPositionInVehicle},
    person_role::{PersonPositionRole, PersonRole},
    person_sex::{PersonSex, PersonSexType},
    person_type::{PersonType, PersonTypeType},
    time::Time,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

impl Fact {
    /// Build fact rows — one row per Person, joining to their Crash for measures and time.
    ///
    /// Dimension lookups use the pre-built dimension tables so that every FK is valid.
    #[allow(clippy::too_many_arguments)]
    pub fn gen_facts(
        persons: Vec<Person>,
        crashes: Vec<Crash>,
        dim_times: &[Time],
        dim_ages: &[PersonAge],
        dim_positions: &[PersonPosition],
        dim_roles: &[PersonPositionRole],
        dim_sexes: &[PersonSex],
        dim_types: &[PersonType],
        dim_factors: &[ContributingFactorDim],
    ) -> Vec<Fact> {
        // Index dimension tables by their natural keys for O(1) lookup.
        let age_by_age: HashMap<u8, u32> = dim_ages
            .iter()
            .filter(|a| a.person_age_known)
            .map(|a| (a.person_age, a.person_age_id))
            .collect();

        let unknown_age_id = dim_ages
            .iter()
            .find(|a| !a.person_age_known)
            .map(|a| a.person_age_id)
            .unwrap_or(0);

        let position_by_type: HashMap<PersonPositionInVehicle, u32> = dim_positions
            .iter()
            .map(|p| (p.person_position, p.person_position_id))
            .collect();

        let role_by_type: HashMap<PersonRole, u32> = dim_roles
            .iter()
            .map(|r| (r.person_position_role, r.person_position_role_id))
            .collect();

        let sex_by_type: HashMap<PersonSexType, u32> = dim_sexes
            .iter()
            .map(|s| (s.person_sex, s.person_sex_id))
            .collect();

        let type_by_type: HashMap<PersonTypeType, u32> = dim_types
            .iter()
            .map(|t| (t.person_type, t.person_type_id))
            .collect();

        let factor_by_factor: HashMap<ContributingFactor, u32> = dim_factors
            .iter()
            .map(|f| (f.contributing_factor, f.contributing_factor_id))
            .collect();

        // Index crashes by crash_id.
        let crash_by_id: HashMap<u32, &Crash> = crashes.iter().map(|c| (c.crash_id, c)).collect();

        // The data-mart Time dimension is 1-indexed and ordered by timestamp.
        // Build a lookup from the base-database time_id → data-mart time_id.
        // The data mart Time::gen_times() enumerates base-db times in order and assigns
        // time_id = index + 1, so the mapping is simply: dm_time_id = bdb_time_id + 1.
        // We confirm this via the dim_times slice rather than assuming.
        // For safety, build a reverse map timestamp → dm_time_id.
        let dm_time_by_bdb_time_id: HashMap<u32, u32> = dim_times
            .iter()
            .enumerate()
            .map(|(i, t)| (i as u32, t.time_id)) // bdb times were 0-indexed
            .collect();

        // Secondary O(1) fallback: look up dm time_id directly by hour-truncated timestamp.
        // This avoids an O(n) linear scan for crashes whose bdb time_id wasn't matched
        // (e.g. crashes outside the weather data range).
        let dm_time_by_timestamp: HashMap<time::OffsetDateTime, u32> =
            dim_times.iter().map(|t| (t.timestamp, t.time_id)).collect();

        persons
            .into_iter()
            .filter_map(|person| {
                let crash = crash_by_id.get(&person.crash_id)?;

                // Resolve time_id: crash carries a bdb time_id; map to dm time_id.
                let time_id: u32 = crash
                    .time_id
                    .and_then(|bdb_id| dm_time_by_bdb_time_id.get(&bdb_id).copied())
                    .or_else(|| {
                        // Fall back: look up the dm time by hour-truncated timestamp (O(1)).
                        let crash_hour = crash
                            .crash_timestamp
                            .replace_minute(0)
                            .and_then(|t| t.replace_second(0))
                            .and_then(|t| t.replace_nanosecond(0))
                            .ok()?;
                        dm_time_by_timestamp.get(&crash_hour).copied()
                    })?;

                // Resolve contributing factor id.
                let cf = crash
                    .crash_factor
                    .map(base_crash_factor_to_dm)
                    .unwrap_or(ContributingFactor::Unknown);
                let contributing_factor_id = *factor_by_factor.get(&cf).unwrap_or(&0);

                // Resolve person age id.
                let person_age_id = person
                    .person_age
                    .and_then(|age| age_by_age.get(&age).copied())
                    .unwrap_or(unknown_age_id);

                // Resolve person position id.
                let position_dm = person
                    .person_position_in_vehicle
                    .map(base_position_to_dm)
                    .unwrap_or(PersonPositionInVehicle::Unknown);
                let person_position_id = *position_by_type.get(&position_dm).unwrap_or(&0);

                // Resolve person role id.
                let role_dm = person
                    .person_role
                    .map(base_role_to_dm)
                    .unwrap_or(PersonRole::Unknown);
                let person_role_id = *role_by_type.get(&role_dm).unwrap_or(&0);

                // Resolve person sex id.
                let sex_dm = person
                    .person_sex
                    .map(base_sex_to_dm)
                    .unwrap_or(PersonSexType::Unknown);
                let person_sex_id = *sex_by_type.get(&sex_dm).unwrap_or(&0);

                // Resolve person type id.
                let type_dm = person
                    .person_type
                    .map(base_type_to_dm)
                    .unwrap_or(PersonTypeType::Unknown);
                let person_type_id = *type_by_type.get(&type_dm).unwrap_or(&0);

                Some(Fact {
                    contributing_factor_id,
                    person_age_id,
                    person_position_id,
                    person_role_id,
                    person_sex_id,
                    person_type_id,
                    time_id,
                    persons_injured: crash.crash_persons_injured.min(u8::MAX as u16) as u8,
                    persons_killed: crash.crash_persons_killed.min(u8::MAX as u16) as u8,
                    pedestrians_injured: crash.crash_pedestrians_injured.min(u8::MAX as u16) as u8,
                    pedestrians_killed: crash.crash_pedestrians_killed.min(u8::MAX as u16) as u8,
                    cyclist_injured: crash.crash_cyclist_injured.min(u8::MAX as u16) as u8,
                    cyclist_killed: crash.crash_cyclist_killed.min(u8::MAX as u16) as u8,
                    motorist_injured: crash.crash_motorist_injured.min(u8::MAX as u16) as u8,
                    motorist_killed: crash.crash_motorist_killed.min(u8::MAX as u16) as u8,
                })
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Mapping helpers: base_database enums → data_mart enums
// ---------------------------------------------------------------------------

fn base_crash_factor_to_dm(f: crate::base_database::crash::CrashFactor) -> ContributingFactor {
    use crate::base_database::crash::CrashFactor as B;
    match f {
        B::DriverlessRunawayVehicle => ContributingFactor::DriverlessRunawayVehicle,
        B::ListeningUsingHeadphones => ContributingFactor::ListeningUsingHeadphones,
        B::EatingOrDrinking => ContributingFactor::EatingOrDrinking,
        B::UnsafeLaneChanging => ContributingFactor::UnsafeLaneChanging,
        B::CellPhoneHandHeld => ContributingFactor::CellPhoneHandHeld,
        B::CellPhoneHandsFree => ContributingFactor::CellPhoneHandsFree,
        B::DrugsIllegal => ContributingFactor::DrugsIllegal,
        B::Texting => ContributingFactor::Texting,
        B::HeadlightsDefective => ContributingFactor::HeadlightsDefective,
        B::OtherLightingDefects => ContributingFactor::OtherLightingDefects,
        B::DriverInexperience => ContributingFactor::DriverInexperience,
        B::AggressiveDrivingRoadRage => ContributingFactor::AggressiveDrivingRoadRage,
        B::UnsafeSpeed => ContributingFactor::UnsafeSpeed,
        B::LaneMarkingImproperInadequate => ContributingFactor::LaneMarkingImproperInadequate,
        B::Glare => ContributingFactor::Glare,
        B::TrafficControlDeviceImproperNonWorking => {
            ContributingFactor::TrafficControlDeviceImproperNonWorking
        }
        B::PassingTooClosely => ContributingFactor::PassingTooClosely,
        B::AcceleratorDefective => ContributingFactor::AcceleratorDefective,
        B::ShouldersDefectiveImproper => ContributingFactor::ShouldersDefectiveImproper,
        B::OutsideCarDistraction => ContributingFactor::OutsideCarDistraction,
        B::DriverInattentionDistraction => ContributingFactor::DriverInattentionDistraction,
        B::TintedWindows => ContributingFactor::TintedWindows,
        B::UsingOnBoardNavigationDevice => ContributingFactor::UsingOnBoardNavigationDevice,
        B::ReactionToOtherUninvolvedVehicle => ContributingFactor::ReactionToOtherUninvolvedVehicle,
        B::ObstructionDebris => ContributingFactor::ObstructionDebris,
        B::PrescriptionMedication => ContributingFactor::PrescriptionMedication,
        B::TireFailureInadequate => ContributingFactor::TireFailureInadequate,
        B::FatiguedDrowsy => ContributingFactor::FatiguedDrowsy,
        B::PassingOrLaneUsageImproper => ContributingFactor::PassingOrLaneUsageImproper,
        B::FollowingTooClosely => ContributingFactor::FollowingTooClosely,
        B::ViewObstructedLimited => ContributingFactor::ViewObstructedLimited,
        B::OversizedVehicle => ContributingFactor::OversizedVehicle,
        B::LostConsciousness => ContributingFactor::LostConsciousness,
        B::BackingUnsafely => ContributingFactor::BackingUnsafely,
        B::OtherVehicular => ContributingFactor::OtherVehicular,
        B::Illness => ContributingFactor::Illness,
        B::WindshieldInadequate => ContributingFactor::WindshieldInadequate,
        B::FellAsleep => ContributingFactor::FellAsleep,
        B::TrafficControlDisregarded => ContributingFactor::TrafficControlDisregarded,
        B::PavementDefective => ContributingFactor::PavementDefective,
        B::SteeringFailure => ContributingFactor::SteeringFailure,
        B::PassengerDistraction => ContributingFactor::PassengerDistraction,
        B::VehicleVandalism => ContributingFactor::VehicleVandalism,
        B::FailureToKeepRight => ContributingFactor::FailureToKeepRight,
        B::BrakesDefective => ContributingFactor::BrakesDefective,
        B::TurningImproperly => ContributingFactor::TurningImproperly,
        B::FailureToYieldRightOfWay => ContributingFactor::FailureToYieldRightOfWay,
        B::ReactionToUninvolvedVehicle => ContributingFactor::ReactionToUninvolvedVehicle,
        B::TowHitchDefective => ContributingFactor::TowHitchDefective,
        B::AlcoholInvolvement => ContributingFactor::AlcoholInvolvement,
        B::PhysicalDisability => ContributingFactor::PhysicalDisability,
        B::AnimalsAction => ContributingFactor::AnimalsAction,
        B::OtherElectronicDevice => ContributingFactor::OtherElectronicDevice,
        B::PedestrianBicyclistOtherPedestrianErrorConfusion => {
            ContributingFactor::PedestrianBicyclistOtherPedestrianErrorConfusion
        }
        B::PavementSlippery => ContributingFactor::PavementSlippery,
    }
}

fn base_position_to_dm(
    p: crate::base_database::person::PersonPositionInVehicle,
) -> PersonPositionInVehicle {
    use crate::base_database::person::PersonPositionInVehicle as B;
    match p {
        B::Driver => PersonPositionInVehicle::Driver,
        B::Front => PersonPositionInVehicle::Front,
        B::Rear => PersonPositionInVehicle::Rear,
        B::Lap => PersonPositionInVehicle::Lap,
        B::Outside => PersonPositionInVehicle::Outside,
    }
}

fn base_role_to_dm(r: crate::base_database::person::PersonRole) -> PersonRole {
    use crate::base_database::person::PersonRole as B;
    match r {
        B::NotifiedPerson => PersonRole::NotifiedPerson,
        B::Witness => PersonRole::Witness,
        B::Registrant => PersonRole::Registrant,
        B::InLineSkater => PersonRole::InLineSkater,
        B::Passenger => PersonRole::Passenger,
        B::Driver => PersonRole::Driver,
        B::PolicyHolder => PersonRole::PolicyHolder,
        B::Owner => PersonRole::Owner,
        B::Pedestrian => PersonRole::Pedestrian,
    }
}

fn base_sex_to_dm(s: crate::base_database::person::PersonSex) -> PersonSexType {
    use crate::base_database::person::PersonSex as B;
    match s {
        B::Male => PersonSexType::Male,
        B::Female => PersonSexType::Female,
    }
}

fn base_type_to_dm(t: crate::base_database::person::PersonType) -> PersonTypeType {
    use crate::base_database::person::PersonType as B;
    match t {
        B::Pedestrian => PersonTypeType::Pedestrian,
        B::Occupant => PersonTypeType::Occupant,
        B::Bicyclist => PersonTypeType::Bicyclist,
        B::OtherMotorized => PersonTypeType::OtherMotorized,
    }
}
