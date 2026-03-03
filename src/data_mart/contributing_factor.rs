use serde::{Deserialize, Serialize};
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumCountMacro)]
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
    Unknown,
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
    Unknown,
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
    Unknown,
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
    Unknown,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubstanceRelatedFactor {
    AlcoholInvolvement,
    DrugsIllegal,
    PrescriptionMedication,
    Unknown,
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
    Unknown,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvironmentalFactor {
    Glare,
    ObstructionDebris,
    AnimalsAction,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalFactor {
    ReactionToOtherUninvolvedVehicle,
    ReactionToUninvolvedVehicle,
    PedestrianBicyclistOtherPedestrianErrorConfusion,
    Unknown,
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
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContributingFactorDim {
    pub contributing_factor_id: u32,
    pub contributing_factor: ContributingFactor,
    pub contributing_factor_hier_def_category: ContributingFactorCategory,
    pub contributing_factor_hier_def_subcategory: ContributingFactorHierarchy,
}

impl ContributingFactorDim {
    pub fn gen_factors() -> Vec<ContributingFactorDim> {
        vec![
            ContributingFactorDim {
                contributing_factor_id: 0,
                contributing_factor: ContributingFactor::Unknown,
                contributing_factor_hier_def_category: ContributingFactorCategory::Unknown,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Unknown,
            },
            ContributingFactorDim {
                contributing_factor_id: 1,
                contributing_factor: ContributingFactor::DriverlessRunawayVehicle,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::DriverlessRunawayVehicle,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 2,
                contributing_factor: ContributingFactor::ListeningUsingHeadphones,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::ListeningUsingHeadphones,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 3,
                contributing_factor: ContributingFactor::EatingOrDrinking,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::EatingOrDrinking,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 4,
                contributing_factor: ContributingFactor::UnsafeLaneChanging,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::UnsafeLaneChanging,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 5,
                contributing_factor: ContributingFactor::CellPhoneHandHeld,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::CellPhoneHandHeld,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 6,
                contributing_factor: ContributingFactor::CellPhoneHandsFree,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::CellPhoneHandsFree,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 7,
                contributing_factor: ContributingFactor::DrugsIllegal,
                contributing_factor_hier_def_category: ContributingFactorCategory::SubstanceRelated,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::SubstanceRelated(
                        SubstanceRelatedFactor::DrugsIllegal,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 8,
                contributing_factor: ContributingFactor::Texting,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::Texting,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 9,
                contributing_factor: ContributingFactor::HeadlightsDefective,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::HeadlightsDefective,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 10,
                contributing_factor: ContributingFactor::OtherLightingDefects,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::OtherLightingDefects,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 11,
                contributing_factor: ContributingFactor::DriverInexperience,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanCondition,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanCondition(
                        HumanConditionFactor::DriverInexperience,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 12,
                contributing_factor: ContributingFactor::AggressiveDrivingRoadRage,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::AggressiveDrivingRoadRage,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 13,
                contributing_factor: ContributingFactor::UnsafeSpeed,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(HumanBehaviorFactor::UnsafeSpeed),
            },
            ContributingFactorDim {
                contributing_factor_id: 14,
                contributing_factor: ContributingFactor::LaneMarkingImproperInadequate,
                contributing_factor_hier_def_category:
                    ContributingFactorCategory::RoadInfrastructure,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::RoadInfrastructure(
                        RoadInfrastructureFactor::LaneMarkingImproperInadequate,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 15,
                contributing_factor: ContributingFactor::Glare,
                contributing_factor_hier_def_category: ContributingFactorCategory::Environmental,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::Environmental(EnvironmentalFactor::Glare),
            },
            ContributingFactorDim {
                contributing_factor_id: 16,
                contributing_factor: ContributingFactor::TrafficControlDeviceImproperNonWorking,
                contributing_factor_hier_def_category:
                    ContributingFactorCategory::RoadInfrastructure,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::RoadInfrastructure(
                        RoadInfrastructureFactor::TrafficControlDeviceImproperNonWorking,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 17,
                contributing_factor: ContributingFactor::PassingTooClosely,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::PassingTooClosely,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 18,
                contributing_factor: ContributingFactor::AcceleratorDefective,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::AcceleratorDefective,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 19,
                contributing_factor: ContributingFactor::ShouldersDefectiveImproper,
                contributing_factor_hier_def_category:
                    ContributingFactorCategory::RoadInfrastructure,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::RoadInfrastructure(
                        RoadInfrastructureFactor::ShouldersDefectiveImproper,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 20,
                contributing_factor: ContributingFactor::OutsideCarDistraction,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::OutsideCarDistraction,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 21,
                contributing_factor: ContributingFactor::DriverInattentionDistraction,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::DriverInattentionDistraction,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 22,
                contributing_factor: ContributingFactor::TintedWindows,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(VehicleDefectFactor::TintedWindows),
            },
            ContributingFactorDim {
                contributing_factor_id: 23,
                contributing_factor: ContributingFactor::UsingOnBoardNavigationDevice,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::UsingOnBoardNavigationDevice,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 24,
                contributing_factor: ContributingFactor::ReactionToOtherUninvolvedVehicle,
                contributing_factor_hier_def_category: ContributingFactorCategory::External,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::External(
                    ExternalFactor::ReactionToOtherUninvolvedVehicle,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 25,
                contributing_factor: ContributingFactor::ObstructionDebris,
                contributing_factor_hier_def_category: ContributingFactorCategory::Environmental,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::Environmental(
                        EnvironmentalFactor::ObstructionDebris,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 26,
                contributing_factor: ContributingFactor::PrescriptionMedication,
                contributing_factor_hier_def_category: ContributingFactorCategory::SubstanceRelated,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::SubstanceRelated(
                        SubstanceRelatedFactor::PrescriptionMedication,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 27,
                contributing_factor: ContributingFactor::TireFailureInadequate,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::TireFailureInadequate,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 28,
                contributing_factor: ContributingFactor::FatiguedDrowsy,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanCondition,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanCondition(
                        HumanConditionFactor::FatiguedDrowsy,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 29,
                contributing_factor: ContributingFactor::PassingOrLaneUsageImproper,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::PassingOrLaneUsageImproper,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 30,
                contributing_factor: ContributingFactor::FollowingTooClosely,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::FollowingTooClosely,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 31,
                contributing_factor: ContributingFactor::ViewObstructedLimited,
                contributing_factor_hier_def_category:
                    ContributingFactorCategory::RoadInfrastructure,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::RoadInfrastructure(
                        RoadInfrastructureFactor::ViewObstructedLimited,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 32,
                contributing_factor: ContributingFactor::OversizedVehicle,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::OversizedVehicle,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 33,
                contributing_factor: ContributingFactor::LostConsciousness,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanCondition,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanCondition(
                        HumanConditionFactor::LostConsciousness,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 34,
                contributing_factor: ContributingFactor::BackingUnsafely,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(HumanBehaviorFactor::BackingUnsafely),
            },
            ContributingFactorDim {
                contributing_factor_id: 35,
                contributing_factor: ContributingFactor::OtherVehicular,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(VehicleDefectFactor::OtherVehicular),
            },
            ContributingFactorDim {
                contributing_factor_id: 36,
                contributing_factor: ContributingFactor::Illness,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanCondition,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanCondition(HumanConditionFactor::Illness),
            },
            ContributingFactorDim {
                contributing_factor_id: 37,
                contributing_factor: ContributingFactor::WindshieldInadequate,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::WindshieldInadequate,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 38,
                contributing_factor: ContributingFactor::FellAsleep,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanCondition,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanCondition(HumanConditionFactor::FellAsleep),
            },
            ContributingFactorDim {
                contributing_factor_id: 39,
                contributing_factor: ContributingFactor::TrafficControlDisregarded,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::TrafficControlDisregarded,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 40,
                contributing_factor: ContributingFactor::PavementDefective,
                contributing_factor_hier_def_category:
                    ContributingFactorCategory::RoadInfrastructure,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::RoadInfrastructure(
                        RoadInfrastructureFactor::PavementDefective,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 41,
                contributing_factor: ContributingFactor::SteeringFailure,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(VehicleDefectFactor::SteeringFailure),
            },
            ContributingFactorDim {
                contributing_factor_id: 42,
                contributing_factor: ContributingFactor::PassengerDistraction,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::PassengerDistraction,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 43,
                contributing_factor: ContributingFactor::VehicleVandalism,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::VehicleVandalism,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 44,
                contributing_factor: ContributingFactor::FailureToKeepRight,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::FailureToKeepRight,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 45,
                contributing_factor: ContributingFactor::BrakesDefective,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(VehicleDefectFactor::BrakesDefective),
            },
            ContributingFactorDim {
                contributing_factor_id: 46,
                contributing_factor: ContributingFactor::TurningImproperly,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::TurningImproperly,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 47,
                contributing_factor: ContributingFactor::FailureToYieldRightOfWay,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanBehavior,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanBehavior(
                        HumanBehaviorFactor::FailureToYieldRightOfWay,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 48,
                contributing_factor: ContributingFactor::ReactionToUninvolvedVehicle,
                contributing_factor_hier_def_category: ContributingFactorCategory::External,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::External(
                    ExternalFactor::ReactionToUninvolvedVehicle,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 49,
                contributing_factor: ContributingFactor::TowHitchDefective,
                contributing_factor_hier_def_category: ContributingFactorCategory::VehicleDefect,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::VehicleDefect(
                        VehicleDefectFactor::TowHitchDefective,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 50,
                contributing_factor: ContributingFactor::AlcoholInvolvement,
                contributing_factor_hier_def_category: ContributingFactorCategory::SubstanceRelated,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::SubstanceRelated(
                        SubstanceRelatedFactor::AlcoholInvolvement,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 51,
                contributing_factor: ContributingFactor::PhysicalDisability,
                contributing_factor_hier_def_category: ContributingFactorCategory::HumanCondition,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::HumanCondition(
                        HumanConditionFactor::PhysicalDisability,
                    ),
            },
            ContributingFactorDim {
                contributing_factor_id: 52,
                contributing_factor: ContributingFactor::AnimalsAction,
                contributing_factor_hier_def_category: ContributingFactorCategory::Environmental,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::Environmental(EnvironmentalFactor::AnimalsAction),
            },
            ContributingFactorDim {
                contributing_factor_id: 53,
                contributing_factor: ContributingFactor::OtherElectronicDevice,
                contributing_factor_hier_def_category: ContributingFactorCategory::Distraction,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::Distraction(
                    DistractionFactor::OtherElectronicDevice,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 54,
                contributing_factor:
                    ContributingFactor::PedestrianBicyclistOtherPedestrianErrorConfusion,
                contributing_factor_hier_def_category: ContributingFactorCategory::External,
                contributing_factor_hier_def_subcategory: ContributingFactorHierarchy::External(
                    ExternalFactor::PedestrianBicyclistOtherPedestrianErrorConfusion,
                ),
            },
            ContributingFactorDim {
                contributing_factor_id: 55,
                contributing_factor: ContributingFactor::PavementSlippery,
                contributing_factor_hier_def_category:
                    ContributingFactorCategory::RoadInfrastructure,
                contributing_factor_hier_def_subcategory:
                    ContributingFactorHierarchy::RoadInfrastructure(
                        RoadInfrastructureFactor::PavementSlippery,
                    ),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::EnumCount;

    #[test]
    fn gen_factors_length_matches_enum_count() {
        assert_eq!(
            ContributingFactorDim::gen_factors().len(),
            ContributingFactor::COUNT,
        );
    }
}
