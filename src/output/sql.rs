use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufWriter, Write as IoWrite};

use crate::data_mart::{
    contributing_factor::{
        ContributingFactorCategory, ContributingFactorDim, ContributingFactorHierarchy,
    },
    fact::Fact,
    person_age::{PersonAge, PersonAgeGroup},
    person_position::{PersonPosition, PersonPositionInVehicle},
    person_role::{PersonPositionRole, PersonRole},
    person_sex::{PersonSex, PersonSexType},
    person_type::{PersonType, PersonTypeType},
    time::{MoonPhase, Time, Weather},
};

// ---------------------------------------------------------------------------
// Batch size: number of value-tuples per INSERT statement.
// Large enough to keep file size down, small enough to avoid parser limits.
// ---------------------------------------------------------------------------
const BATCH_SIZE: usize = 1_000;

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

pub fn write_dim_time(path: &str, rows: &[Time]) {
    let columns = "time_id, [timestamp], hier_def_day, hier_def_month, hier_def_year, hier_moon_phase, weather";
    let table = "dm.DimTime";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, '{}', '{}', '{}', {}, '{}', '{}')",
            row.time_id,
            format_offset_datetime(row.timestamp),
            row.hier_def_day,
            row.hier_def_month,
            row.hier_def_year,
            moon_phase_str(row.hier_moon_phase),
            weather_str(row.weather),
        )
        .unwrap();
    });
}

pub fn write_dim_person_age(path: &str, rows: &[PersonAge]) {
    let columns = "person_age_id, person_age, person_age_known, person_age_hier_def_group";
    let table = "dm.DimPersonAge";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, {}, {}, '{}')",
            row.person_age_id,
            row.person_age,
            sql_bit(row.person_age_known),
            age_group_str(row.person_age_hier_def_group),
        )
        .unwrap();
    });
}

pub fn write_dim_person_position(path: &str, rows: &[PersonPosition]) {
    let columns = "person_position_id, person_position";
    let table = "dm.DimPersonPosition";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, '{}')",
            row.person_position_id,
            position_str(row.person_position),
        )
        .unwrap();
    });
}

pub fn write_dim_person_role(path: &str, rows: &[PersonPositionRole]) {
    let columns = "person_role_id, person_role";
    let table = "dm.DimPersonRole";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, '{}')",
            row.person_position_role_id,
            role_str(row.person_position_role),
        )
        .unwrap();
    });
}

pub fn write_dim_person_sex(path: &str, rows: &[PersonSex]) {
    let columns = "person_sex_id, person_sex";
    let table = "dm.DimPersonSex";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, '{}')",
            row.person_sex_id,
            sex_str(row.person_sex),
        )
        .unwrap();
    });
}

pub fn write_dim_person_type(path: &str, rows: &[PersonType]) {
    let columns = "person_type_id, person_type";
    let table = "dm.DimPersonType";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, '{}')",
            row.person_type_id,
            type_str(row.person_type),
        )
        .unwrap();
    });
}

pub fn write_dim_contributing_factor(path: &str, rows: &[ContributingFactorDim]) {
    let columns = "contributing_factor_id, contributing_factor, contributing_factor_hier_def_category, contributing_factor_hier_def_subcategory";
    let table = "dm.DimContributingFactor";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, '{}', '{}', '{}')",
            row.contributing_factor_id,
            // SCREAMING_SNAKE_CASE via serde — replicate the same mapping here
            contributing_factor_str(row.contributing_factor),
            category_str(row.contributing_factor_hier_def_category),
            hierarchy_subcategory_str(&row.contributing_factor_hier_def_subcategory),
        )
        .unwrap();
    });
}

pub fn write_fact(path: &str, rows: &[Fact]) {
    let columns = concat!(
        "fact_id, ",
        "contributing_factor_id, person_age_id, person_position_id, person_role_id, ",
        "person_sex_id, person_type_id, time_id, ",
        "persons_injured, persons_killed, ",
        "pedestrians_injured, pedestrians_killed, ",
        "cyclist_injured, cyclist_killed, ",
        "motorist_injured, motorist_killed"
    );
    let table = "dm.Fact";
    write_batched(path, table, columns, rows, |row, buf| {
        write!(
            buf,
            "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            row.fact_id,
            row.contributing_factor_id,
            row.person_age_id,
            row.person_position_id,
            row.person_role_id,
            row.person_sex_id,
            row.person_type_id,
            row.time_id,
            row.persons_injured,
            row.persons_killed,
            row.pedestrians_injured,
            row.pedestrians_killed,
            row.cyclist_injured,
            row.cyclist_killed,
            row.motorist_injured,
            row.motorist_killed,
        )
        .unwrap();
    });
}

// ---------------------------------------------------------------------------
// Core batching writer
// ---------------------------------------------------------------------------

/// Streams `rows` into `path` as batched INSERT statements.
/// Each batch contains at most `BATCH_SIZE` value-tuples.
/// `render` writes a single value-tuple into the provided `String` buffer.
fn write_batched<T, F>(path: &str, table: &str, columns: &str, rows: &[T], render: F)
where
    F: Fn(&T, &mut String),
{
    let file = File::create(path).unwrap_or_else(|e| panic!("failed to create {path}: {e}"));
    let mut writer = BufWriter::new(file);

    let mut buf = String::with_capacity(256);

    for chunk in rows.chunks(BATCH_SIZE) {
        writer
            .write_all(format!("INSERT INTO {table}\n    ({columns})\nVALUES\n").as_bytes())
            .unwrap();

        for (i, row) in chunk.iter().enumerate() {
            buf.clear();
            buf.push_str("    ");
            render(row, &mut buf);
            if i < chunk.len() - 1 {
                buf.push(',');
            } else {
                buf.push(';');
            }
            buf.push('\n');
            writer.write_all(buf.as_bytes()).unwrap();
        }

        writer.write_all(b"\n").unwrap();
    }

    writer
        .flush()
        .unwrap_or_else(|e| panic!("failed to flush {path}: {e}"));
}

// ---------------------------------------------------------------------------
// Value-formatting helpers
// ---------------------------------------------------------------------------

/// Format an `OffsetDateTime` as an RFC3339 string.
/// e.g. `2021-07-04T13:00:00Z`
/// All timestamps in this project are UTC, so the offset is always `Z`.
fn format_offset_datetime(dt: time::OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        dt.year(),
        dt.month() as u8,
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    )
}

fn sql_bit(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

fn moon_phase_str(p: MoonPhase) -> &'static str {
    match p {
        MoonPhase::New => "NEW",
        MoonPhase::WaxingCrescent => "WAXING_CRESCENT",
        MoonPhase::FirstQuarter => "FIRST_QUARTER",
        MoonPhase::WaxingGibbous => "WAXING_GIBBOUS",
        MoonPhase::Full => "FULL",
        MoonPhase::WaningGibbous => "WANING_GIBBOUS",
        MoonPhase::LastQuarter => "LAST_QUARTER",
        MoonPhase::WaningCrescent => "WANING_CRESCENT",
        MoonPhase::Unknown => "UNKNOWN",
    }
}

fn weather_str(w: Weather) -> &'static str {
    match w {
        Weather::Clear => "CLEAR",
        Weather::Cloudy => "CLOUDY",
        Weather::RainyLight => "RAINY_LIGHT",
        Weather::RainyHeavy => "RAINY_HEAVY",
        Weather::Stormy => "STORMY",
        Weather::Windy => "WINDY",
        Weather::Miscallaneous => "MISCALLANEOUS",
        Weather::Unknown => "UNKNOWN",
    }
}

fn age_group_str(g: PersonAgeGroup) -> &'static str {
    match g {
        PersonAgeGroup::Fertile => "FERTILE",
        PersonAgeGroup::Infertile => "INFERTILE",
        PersonAgeGroup::Unknown => "UNKNOWN",
    }
}

fn position_str(p: PersonPositionInVehicle) -> &'static str {
    match p {
        PersonPositionInVehicle::Driver => "DRIVER",
        PersonPositionInVehicle::Front => "FRONT",
        PersonPositionInVehicle::Rear => "REAR",
        PersonPositionInVehicle::Lap => "LAP",
        PersonPositionInVehicle::Outside => "OUTSIDE",
        PersonPositionInVehicle::Unknown => "UNKNOWN",
    }
}

fn role_str(r: PersonRole) -> &'static str {
    match r {
        PersonRole::NotifiedPerson => "NOTIFIED_PERSON",
        PersonRole::Witness => "WITNESS",
        PersonRole::Registrant => "REGISTRANT",
        PersonRole::InLineSkater => "IN_LINE_SKATER",
        PersonRole::Passenger => "PASSENGER",
        PersonRole::Driver => "DRIVER",
        PersonRole::PolicyHolder => "POLICY_HOLDER",
        PersonRole::Owner => "OWNER",
        PersonRole::Pedestrian => "PEDESTRIAN",
        PersonRole::Unknown => "UNKNOWN",
    }
}

fn sex_str(s: PersonSexType) -> &'static str {
    match s {
        PersonSexType::Male => "MALE",
        PersonSexType::Female => "FEMALE",
        PersonSexType::Unknown => "UNKNOWN",
    }
}

fn type_str(t: PersonTypeType) -> &'static str {
    match t {
        PersonTypeType::Pedestrian => "PEDESTRIAN",
        PersonTypeType::Occupant => "OCCUPANT",
        PersonTypeType::Bicyclist => "BICYCLIST",
        PersonTypeType::OtherMotorized => "OTHER_MOTORIZED",
        PersonTypeType::Unknown => "UNKNOWN",
    }
}

fn contributing_factor_str(
    f: crate::data_mart::contributing_factor::ContributingFactor,
) -> &'static str {
    use crate::data_mart::contributing_factor::ContributingFactor as CF;
    match f {
        CF::DriverlessRunawayVehicle => "DRIVERLESS_RUNAWAY_VEHICLE",
        CF::ListeningUsingHeadphones => "LISTENING_USING_HEADPHONES",
        CF::EatingOrDrinking => "EATING_OR_DRINKING",
        CF::UnsafeLaneChanging => "UNSAFE_LANE_CHANGING",
        CF::CellPhoneHandHeld => "CELL_PHONE_HAND_HELD",
        CF::CellPhoneHandsFree => "CELL_PHONE_HANDS_FREE",
        CF::DrugsIllegal => "DRUGS_ILLEGAL",
        CF::Texting => "TEXTING",
        CF::HeadlightsDefective => "HEADLIGHTS_DEFECTIVE",
        CF::OtherLightingDefects => "OTHER_LIGHTING_DEFECTS",
        CF::DriverInexperience => "DRIVER_INEXPERIENCE",
        CF::AggressiveDrivingRoadRage => "AGGRESSIVE_DRIVING_ROAD_RAGE",
        CF::UnsafeSpeed => "UNSAFE_SPEED",
        CF::LaneMarkingImproperInadequate => "LANE_MARKING_IMPROPER_INADEQUATE",
        CF::Glare => "GLARE",
        CF::TrafficControlDeviceImproperNonWorking => "TRAFFIC_CONTROL_DEVICE_IMPROPER_NON_WORKING",
        CF::PassingTooClosely => "PASSING_TOO_CLOSELY",
        CF::AcceleratorDefective => "ACCELERATOR_DEFECTIVE",
        CF::ShouldersDefectiveImproper => "SHOULDERS_DEFECTIVE_IMPROPER",
        CF::OutsideCarDistraction => "OUTSIDE_CAR_DISTRACTION",
        CF::DriverInattentionDistraction => "DRIVER_INATTENTION_DISTRACTION",
        CF::TintedWindows => "TINTED_WINDOWS",
        CF::UsingOnBoardNavigationDevice => "USING_ON_BOARD_NAVIGATION_DEVICE",
        CF::ReactionToOtherUninvolvedVehicle => "REACTION_TO_OTHER_UNINVOLVED_VEHICLE",
        CF::ObstructionDebris => "OBSTRUCTION_DEBRIS",
        CF::PrescriptionMedication => "PRESCRIPTION_MEDICATION",
        CF::TireFailureInadequate => "TIRE_FAILURE_INADEQUATE",
        CF::FatiguedDrowsy => "FATIGUED_DROWSY",
        CF::PassingOrLaneUsageImproper => "PASSING_OR_LANE_USAGE_IMPROPER",
        CF::FollowingTooClosely => "FOLLOWING_TOO_CLOSELY",
        CF::ViewObstructedLimited => "VIEW_OBSTRUCTED_LIMITED",
        CF::OversizedVehicle => "OVERSIZED_VEHICLE",
        CF::LostConsciousness => "LOST_CONSCIOUSNESS",
        CF::BackingUnsafely => "BACKING_UNSAFELY",
        CF::OtherVehicular => "OTHER_VEHICULAR",
        CF::Illness => "ILLNESS",
        CF::WindshieldInadequate => "WINDSHIELD_INADEQUATE",
        CF::FellAsleep => "FELL_ASLEEP",
        CF::TrafficControlDisregarded => "TRAFFIC_CONTROL_DISREGARDED",
        CF::PavementDefective => "PAVEMENT_DEFECTIVE",
        CF::SteeringFailure => "STEERING_FAILURE",
        CF::PassengerDistraction => "PASSENGER_DISTRACTION",
        CF::VehicleVandalism => "VEHICLE_VANDALISM",
        CF::FailureToKeepRight => "FAILURE_TO_KEEP_RIGHT",
        CF::BrakesDefective => "BRAKES_DEFECTIVE",
        CF::TurningImproperly => "TURNING_IMPROPERLY",
        CF::FailureToYieldRightOfWay => "FAILURE_TO_YIELD_RIGHT_OF_WAY",
        CF::ReactionToUninvolvedVehicle => "REACTION_TO_UNINVOLVED_VEHICLE",
        CF::TowHitchDefective => "TOW_HITCH_DEFECTIVE",
        CF::AlcoholInvolvement => "ALCOHOL_INVOLVEMENT",
        CF::PhysicalDisability => "PHYSICAL_DISABILITY",
        CF::AnimalsAction => "ANIMALS_ACTION",
        CF::OtherElectronicDevice => "OTHER_ELECTRONIC_DEVICE",
        CF::PedestrianBicyclistOtherPedestrianErrorConfusion => {
            "PEDESTRIAN_BICYCLIST_OTHER_PEDESTRIAN_ERROR_CONFUSION"
        }
        CF::PavementSlippery => "PAVEMENT_SLIPPERY",
        CF::Unknown => "UNKNOWN",
    }
}

fn category_str(c: ContributingFactorCategory) -> &'static str {
    match c {
        ContributingFactorCategory::HumanBehavior => "HUMAN_BEHAVIOR",
        ContributingFactorCategory::HumanCondition => "HUMAN_CONDITION",
        ContributingFactorCategory::Distraction => "DISTRACTION",
        ContributingFactorCategory::SubstanceRelated => "SUBSTANCE_RELATED",
        ContributingFactorCategory::VehicleDefect => "VEHICLE_DEFECT",
        ContributingFactorCategory::RoadInfrastructure => "ROAD_INFRASTRUCTURE",
        ContributingFactorCategory::Environmental => "ENVIRONMENTAL",
        ContributingFactorCategory::External => "EXTERNAL",
        ContributingFactorCategory::Unknown => "UNKNOWN",
    }
}

/// Flatten the `ContributingFactorHierarchy` tagged-union to a plain SCREAMING_SNAKE_CASE string
/// that represents the subcategory-level name (i.e. the inner enum variant).
fn hierarchy_subcategory_str(h: &ContributingFactorHierarchy) -> &'static str {
    use crate::data_mart::contributing_factor::{
        DistractionFactor as DF, EnvironmentalFactor as EF, ExternalFactor as XF,
        HumanBehaviorFactor as HBF, HumanConditionFactor as HCF, RoadInfrastructureFactor as RIF,
        SubstanceRelatedFactor as SRF, VehicleDefectFactor as VDF,
    };
    match h {
        // Human Behavior
        ContributingFactorHierarchy::HumanBehavior(HBF::UnsafeSpeed) => "UNSAFE_SPEED",
        ContributingFactorHierarchy::HumanBehavior(HBF::UnsafeLaneChanging) => {
            "UNSAFE_LANE_CHANGING"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::PassingTooClosely) => "PASSING_TOO_CLOSELY",
        ContributingFactorHierarchy::HumanBehavior(HBF::PassingOrLaneUsageImproper) => {
            "PASSING_OR_LANE_USAGE_IMPROPER"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::FollowingTooClosely) => {
            "FOLLOWING_TOO_CLOSELY"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::FailureToKeepRight) => {
            "FAILURE_TO_KEEP_RIGHT"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::FailureToYieldRightOfWay) => {
            "FAILURE_TO_YIELD_RIGHT_OF_WAY"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::TurningImproperly) => "TURNING_IMPROPERLY",
        ContributingFactorHierarchy::HumanBehavior(HBF::BackingUnsafely) => "BACKING_UNSAFELY",
        ContributingFactorHierarchy::HumanBehavior(HBF::TrafficControlDisregarded) => {
            "TRAFFIC_CONTROL_DISREGARDED"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::AggressiveDrivingRoadRage) => {
            "AGGRESSIVE_DRIVING_ROAD_RAGE"
        }
        ContributingFactorHierarchy::HumanBehavior(HBF::Unknown) => "UNKNOWN",

        // Human Condition
        ContributingFactorHierarchy::HumanCondition(HCF::DriverInexperience) => {
            "DRIVER_INEXPERIENCE"
        }
        ContributingFactorHierarchy::HumanCondition(HCF::FatiguedDrowsy) => "FATIGUED_DROWSY",
        ContributingFactorHierarchy::HumanCondition(HCF::FellAsleep) => "FELL_ASLEEP",
        ContributingFactorHierarchy::HumanCondition(HCF::LostConsciousness) => "LOST_CONSCIOUSNESS",
        ContributingFactorHierarchy::HumanCondition(HCF::Illness) => "ILLNESS",
        ContributingFactorHierarchy::HumanCondition(HCF::PhysicalDisability) => {
            "PHYSICAL_DISABILITY"
        }
        ContributingFactorHierarchy::HumanCondition(HCF::Unknown) => "UNKNOWN",

        // Distraction
        ContributingFactorHierarchy::Distraction(DF::DriverInattentionDistraction) => {
            "DRIVER_INATTENTION_DISTRACTION"
        }
        ContributingFactorHierarchy::Distraction(DF::PassengerDistraction) => {
            "PASSENGER_DISTRACTION"
        }
        ContributingFactorHierarchy::Distraction(DF::OutsideCarDistraction) => {
            "OUTSIDE_CAR_DISTRACTION"
        }
        ContributingFactorHierarchy::Distraction(DF::CellPhoneHandHeld) => "CELL_PHONE_HAND_HELD",
        ContributingFactorHierarchy::Distraction(DF::CellPhoneHandsFree) => "CELL_PHONE_HANDS_FREE",
        ContributingFactorHierarchy::Distraction(DF::Texting) => "TEXTING",
        ContributingFactorHierarchy::Distraction(DF::UsingOnBoardNavigationDevice) => {
            "USING_ON_BOARD_NAVIGATION_DEVICE"
        }
        ContributingFactorHierarchy::Distraction(DF::OtherElectronicDevice) => {
            "OTHER_ELECTRONIC_DEVICE"
        }
        ContributingFactorHierarchy::Distraction(DF::ListeningUsingHeadphones) => {
            "LISTENING_USING_HEADPHONES"
        }
        ContributingFactorHierarchy::Distraction(DF::EatingOrDrinking) => "EATING_OR_DRINKING",
        ContributingFactorHierarchy::Distraction(DF::Unknown) => "UNKNOWN",

        // Substance Related
        ContributingFactorHierarchy::SubstanceRelated(SRF::AlcoholInvolvement) => {
            "ALCOHOL_INVOLVEMENT"
        }
        ContributingFactorHierarchy::SubstanceRelated(SRF::DrugsIllegal) => "DRUGS_ILLEGAL",
        ContributingFactorHierarchy::SubstanceRelated(SRF::PrescriptionMedication) => {
            "PRESCRIPTION_MEDICATION"
        }
        ContributingFactorHierarchy::SubstanceRelated(SRF::Unknown) => "UNKNOWN",

        // Vehicle Defect
        ContributingFactorHierarchy::VehicleDefect(VDF::AcceleratorDefective) => {
            "ACCELERATOR_DEFECTIVE"
        }
        ContributingFactorHierarchy::VehicleDefect(VDF::BrakesDefective) => "BRAKES_DEFECTIVE",
        ContributingFactorHierarchy::VehicleDefect(VDF::SteeringFailure) => "STEERING_FAILURE",
        ContributingFactorHierarchy::VehicleDefect(VDF::TireFailureInadequate) => {
            "TIRE_FAILURE_INADEQUATE"
        }
        ContributingFactorHierarchy::VehicleDefect(VDF::TowHitchDefective) => "TOW_HITCH_DEFECTIVE",
        ContributingFactorHierarchy::VehicleDefect(VDF::HeadlightsDefective) => {
            "HEADLIGHTS_DEFECTIVE"
        }
        ContributingFactorHierarchy::VehicleDefect(VDF::OtherLightingDefects) => {
            "OTHER_LIGHTING_DEFECTS"
        }
        ContributingFactorHierarchy::VehicleDefect(VDF::WindshieldInadequate) => {
            "WINDSHIELD_INADEQUATE"
        }
        ContributingFactorHierarchy::VehicleDefect(VDF::TintedWindows) => "TINTED_WINDOWS",
        ContributingFactorHierarchy::VehicleDefect(VDF::VehicleVandalism) => "VEHICLE_VANDALISM",
        ContributingFactorHierarchy::VehicleDefect(VDF::DriverlessRunawayVehicle) => {
            "DRIVERLESS_RUNAWAY_VEHICLE"
        }
        ContributingFactorHierarchy::VehicleDefect(VDF::OversizedVehicle) => "OVERSIZED_VEHICLE",
        ContributingFactorHierarchy::VehicleDefect(VDF::OtherVehicular) => "OTHER_VEHICULAR",
        ContributingFactorHierarchy::VehicleDefect(VDF::Unknown) => "UNKNOWN",

        // Road Infrastructure
        ContributingFactorHierarchy::RoadInfrastructure(RIF::PavementDefective) => {
            "PAVEMENT_DEFECTIVE"
        }
        ContributingFactorHierarchy::RoadInfrastructure(RIF::PavementSlippery) => {
            "PAVEMENT_SLIPPERY"
        }
        ContributingFactorHierarchy::RoadInfrastructure(RIF::LaneMarkingImproperInadequate) => {
            "LANE_MARKING_IMPROPER_INADEQUATE"
        }
        ContributingFactorHierarchy::RoadInfrastructure(RIF::ShouldersDefectiveImproper) => {
            "SHOULDERS_DEFECTIVE_IMPROPER"
        }
        ContributingFactorHierarchy::RoadInfrastructure(
            RIF::TrafficControlDeviceImproperNonWorking,
        ) => "TRAFFIC_CONTROL_DEVICE_IMPROPER_NON_WORKING",
        ContributingFactorHierarchy::RoadInfrastructure(RIF::ViewObstructedLimited) => {
            "VIEW_OBSTRUCTED_LIMITED"
        }
        ContributingFactorHierarchy::RoadInfrastructure(RIF::Unknown) => "UNKNOWN",

        // Environmental
        ContributingFactorHierarchy::Environmental(EF::Glare) => "GLARE",
        ContributingFactorHierarchy::Environmental(EF::ObstructionDebris) => "OBSTRUCTION_DEBRIS",
        ContributingFactorHierarchy::Environmental(EF::AnimalsAction) => "ANIMALS_ACTION",
        ContributingFactorHierarchy::Environmental(EF::Unknown) => "UNKNOWN",

        // External
        ContributingFactorHierarchy::External(XF::ReactionToOtherUninvolvedVehicle) => {
            "REACTION_TO_OTHER_UNINVOLVED_VEHICLE"
        }
        ContributingFactorHierarchy::External(XF::ReactionToUninvolvedVehicle) => {
            "REACTION_TO_UNINVOLVED_VEHICLE"
        }
        ContributingFactorHierarchy::External(
            XF::PedestrianBicyclistOtherPedestrianErrorConfusion,
        ) => "PEDESTRIAN_BICYCLIST_OTHER_PEDESTRIAN_ERROR_CONFUSION",
        ContributingFactorHierarchy::External(XF::Unknown) => "UNKNOWN",

        ContributingFactorHierarchy::Unknown => "UNKNOWN",
    }
}
