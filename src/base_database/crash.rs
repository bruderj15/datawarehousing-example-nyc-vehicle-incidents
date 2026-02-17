use crate::raw::crashes::RawCrashRecord;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

#[derive(Debug, Clone)]
pub struct Crash {
    pub crash_id: u32,
    pub crash_timestamp: OffsetDateTime,
    pub crash_persons_injured: u16,
    pub crash_persons_killed: u16,
    pub crash_pedestrians_injured: u16,
    pub crash_pedestrians_killed: u16,
    pub crash_cyclist_injured: u16,
    pub crash_cyclist_killed: u16,
    pub crash_motorist_injured: u16,
    pub crash_motorist_killed: u16,
    pub crash_factor_1: Option<CrashFactor>,
    pub crash_factor_2: Option<CrashFactor>,
    pub crash_factor_3: Option<CrashFactor>,
    pub crash_factor_4: Option<CrashFactor>,
    pub crash_factor_5: Option<CrashFactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CrashFactor {
    DriverlessRunawayVehicle,
    ListeningUsingHeadphones,
    EatingOrDrinking,
    UnsafeLaneChanging,
    CellPhoneHandHeld,
    CellPhoneHandsFree,
    DrugsIllegal,
    Texting,
    HeadlightsDefective,
    OtherLightingDefects,
    DriverInexperience,
    AggressiveDrivingRoadRage,
    UnsafeSpeed,
    LaneMarkingImproperInadequate,
    Glare,
    TrafficControlDeviceImproperNonWorking,
    PassingTooClosely,
    AcceleratorDefective,
    ShouldersDefectiveImproper,
    OutsideCarDistraction,
    DriverInattentionDistraction,
    TintedWindows,
    UsingOnBoardNavigationDevice,
    ReactionToOtherUninvolvedVehicle,
    ObstructionDebris,
    PrescriptionMedication,
    TireFailureInadequate,
    FatiguedDrowsy,
    PassingOrLaneUsageImproper,
    FollowingTooClosely,
    ViewObstructedLimited,
    OversizedVehicle,
    LostConsciousness,
    BackingUnsafely,
    OtherVehicular,
    Illness,
    WindshieldInadequate,
    FellAsleep,
    TrafficControlDisregarded,
    PavementDefective,
    SteeringFailure,
    PassengerDistraction,
    VehicleVandalism,
    FailureToKeepRight,
    BrakesDefective,
    TurningImproperly,
    FailureToYieldRightOfWay,
    ReactionToUninvolvedVehicle,
    TowHitchDefective,
    AlcoholInvolvement,
    PhysicalDisability,
    AnimalsAction,
    OtherElectronicDevice,
    PedestrianBicyclistOtherPedestrianErrorConfusion,
    PavementSlippery,
}

impl From<RawCrashRecord> for Crash {
    fn from(raw: RawCrashRecord) -> Self {
        Self {
            crash_id: raw.collision_id,
            crash_timestamp: PrimitiveDateTime::new(raw.crash_date, raw.crash_time)
                .assume_offset(UtcOffset::UTC),
            crash_persons_injured: raw.number_of_persons_injured,
            crash_persons_killed: raw.number_of_persons_killed,
            crash_pedestrians_injured: raw.number_of_pedestrians_injured,
            crash_pedestrians_killed: raw.number_of_pedestrians_killed,
            crash_cyclist_injured: raw.number_of_cyclist_injured,
            crash_cyclist_killed: raw.number_of_cyclist_killed,
            crash_motorist_injured: raw.number_of_motorist_injured,
            crash_motorist_killed: raw.number_of_motorist_killed,
            crash_factor_1: extract_contributing_factor(&raw.contributing_factor_vehicle_1),
            crash_factor_2: extract_contributing_factor(&raw.contributing_factor_vehicle_2),
            crash_factor_3: extract_contributing_factor(&raw.contributing_factor_vehicle_3),
            crash_factor_4: extract_contributing_factor(&raw.contributing_factor_vehicle_4),
            crash_factor_5: extract_contributing_factor(&raw.contributing_factor_vehicle_5),
        }
    }
}

fn extract_contributing_factor(contributing_factor: &str) -> Option<CrashFactor> {
    match contributing_factor.to_lowercase().trim() {
        "driverless/runaway vehicle" => Some(CrashFactor::DriverlessRunawayVehicle),
        "listening/using headphones" => Some(CrashFactor::ListeningUsingHeadphones),
        "eating or drinking" => Some(CrashFactor::EatingOrDrinking),
        "unsafe lane changing" => Some(CrashFactor::UnsafeLaneChanging),
        "cell phone (hand-held)" => Some(CrashFactor::CellPhoneHandHeld),
        "cell phone (hands-free)" => Some(CrashFactor::CellPhoneHandsFree),
        "drugs (illegal)" => Some(CrashFactor::DrugsIllegal),
        "texting" => Some(CrashFactor::Texting),
        "headlights defective" => Some(CrashFactor::HeadlightsDefective),
        "other lighting defects" => Some(CrashFactor::OtherLightingDefects),
        "driver inexperience" => Some(CrashFactor::DriverInexperience),
        "aggressive driving/road rage" => Some(CrashFactor::AggressiveDrivingRoadRage),
        "unsafe speed" => Some(CrashFactor::UnsafeSpeed),
        "lane marking improper/inadequate" => Some(CrashFactor::LaneMarkingImproperInadequate),
        "glare" => Some(CrashFactor::Glare),
        "traffic control device improper/non-working" => {
            Some(CrashFactor::TrafficControlDeviceImproperNonWorking)
        }
        "passing too closely" => Some(CrashFactor::PassingTooClosely),
        "accelerator defective" => Some(CrashFactor::AcceleratorDefective),
        "shoulders defective/improper" => Some(CrashFactor::ShouldersDefectiveImproper),
        "outside car distraction" => Some(CrashFactor::OutsideCarDistraction),
        "driver inattention/distraction" => Some(CrashFactor::DriverInattentionDistraction),
        "tinted windows" => Some(CrashFactor::TintedWindows),
        "using on board navigation device" => Some(CrashFactor::UsingOnBoardNavigationDevice),
        "reaction to other uninvolved vehicle" => {
            Some(CrashFactor::ReactionToOtherUninvolvedVehicle)
        }
        "obstruction/debris" => Some(CrashFactor::ObstructionDebris),
        "prescription medication" => Some(CrashFactor::PrescriptionMedication),
        "tire failure/inadequate" => Some(CrashFactor::TireFailureInadequate),
        "fatigued/drowsy" => Some(CrashFactor::FatiguedDrowsy),
        "passing or lane usage improper" => Some(CrashFactor::PassingOrLaneUsageImproper),
        "following too closely" => Some(CrashFactor::FollowingTooClosely),
        "view obstructed/limited" => Some(CrashFactor::ViewObstructedLimited),
        "oversized vehicle" => Some(CrashFactor::OversizedVehicle),
        "lost consciousness" => Some(CrashFactor::LostConsciousness),
        "backing unsafely" => Some(CrashFactor::BackingUnsafely),
        "other vehicular" => Some(CrashFactor::OtherVehicular),
        "illness" => Some(CrashFactor::Illness),
        "windshield inadequate" => Some(CrashFactor::WindshieldInadequate),
        "fell asleep" => Some(CrashFactor::FellAsleep),
        "traffic control disregarded" => Some(CrashFactor::TrafficControlDisregarded),
        "pavement defective" => Some(CrashFactor::PavementDefective),
        "steering failure" => Some(CrashFactor::SteeringFailure),
        "passenger distraction" => Some(CrashFactor::PassengerDistraction),
        "vehicle vandalism" => Some(CrashFactor::VehicleVandalism),
        "failure to keep right" => Some(CrashFactor::FailureToKeepRight),
        "brakes defective" => Some(CrashFactor::BrakesDefective),
        "turning improperly" => Some(CrashFactor::TurningImproperly),
        "failure to yield right-of-way" => Some(CrashFactor::FailureToYieldRightOfWay),
        "reaction to uninvolved vehicle" => Some(CrashFactor::ReactionToUninvolvedVehicle),
        "tow hitch defective" => Some(CrashFactor::TowHitchDefective),
        "alcohol involvement" => Some(CrashFactor::AlcoholInvolvement),
        "physical disability" => Some(CrashFactor::PhysicalDisability),
        "animals action" => Some(CrashFactor::AnimalsAction),
        "other electronic device" => Some(CrashFactor::OtherElectronicDevice),
        "pedestrian/bicyclist/other pedestrian error/confusion" => {
            Some(CrashFactor::PedestrianBicyclistOtherPedestrianErrorConfusion)
        }
        "pavement slippery" => Some(CrashFactor::PavementSlippery),
        _ => None,
    }
}
