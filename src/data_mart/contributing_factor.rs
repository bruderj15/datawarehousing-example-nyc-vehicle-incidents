use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContributingFactor {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContributingFactorCategory {
    HumanBehavior,
    HumanCondition,
    Distraction,
    SubstanceRelated,
    VehicleDefect,
    RoadInfrastructure,
    Environmental,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HumanBehaviorFactor {
    UnsafeSpeed,
    UnsafeLaneChanging,
    PassingTooClosely,
    PassingOrLaneUsageImproper,
    FollowingTooClosely,
    FailureToKeepRight,
    FailureToYieldRightOfWay,
    TurningImproperly,
    BackingUnsafely,
    TrafficControlDisregarded,
    AggressiveDrivingRoadRage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HumanConditionFactor {
    DriverInexperience,
    FatiguedDrowsy,
    FellAsleep,
    LostConsciousness,
    Illness,
    PhysicalDisability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DistractionFactor {
    DriverInattentionDistraction,
    PassengerDistraction,
    OutsideCarDistraction,
    CellPhoneHandHeld,
    CellPhoneHandsFree,
    Texting,
    UsingOnBoardNavigationDevice,
    OtherElectronicDevice,
    ListeningUsingHeadphones,
    EatingOrDrinking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubstanceRelatedFactor {
    AlcoholInvolvement,
    DrugsIllegal,
    PrescriptionMedication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VehicleDefectFactor {
    AcceleratorDefective,
    BrakesDefective,
    SteeringFailure,
    TireFailureInadequate,
    TowHitchDefective,
    HeadlightsDefective,
    OtherLightingDefects,
    WindshieldInadequate,
    TintedWindows,
    VehicleVandalism,
    DriverlessRunawayVehicle,
    OversizedVehicle,
    OtherVehicular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoadInfrastructureFactor {
    PavementDefective,
    PavementSlippery,
    LaneMarkingImproperInadequate,
    ShouldersDefectiveImproper,
    TrafficControlDeviceImproperNonWorking,
    ViewObstructedLimited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvironmentalFactor {
    Glare,
    ObstructionDebris,
    AnimalsAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalFactor {
    ReactionToOtherUninvolvedVehicle,
    ReactionToUninvolvedVehicle,
    PedestrianBicyclistOtherPedestrianErrorConfusion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContributingFactorHierarchy {
    HumanBehavior(HumanBehaviorFactor),
    HumanCondition(HumanConditionFactor),
    Distraction(DistractionFactor),
    SubstanceRelated(SubstanceRelatedFactor),
    VehicleDefect(VehicleDefectFactor),
    RoadInfrastructure(RoadInfrastructureFactor),
    Environmental(EnvironmentalFactor),
    External(ExternalFactor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContributingFactorDim {
    pub contributing_factor_id: u32,
    pub contributing_factor: ContributingFactor,
    pub contributing_factor_hier_def_category: ContributingFactorCategory,
    pub contributing_factor_hier_def_subcategory: ContributingFactorHierarchy,
}
