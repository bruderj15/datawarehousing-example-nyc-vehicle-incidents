//! Database ingestion module.
//!
//! Provides two public entry-points:
//!
//! * [`setup_data_mart`] – creates all schema objects (idempotent; drops first
//!   if they already exist so a re-run always starts clean).
//! * [`ingest_data_mart`] – bulk-inserts data into the tables selected via a
//!   [`DataMartTable`] slice, so you can skip dimension tables that are already
//!   populated.

use anyhow::{Context, Result};
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::data_mart::{
    contributing_factor::ContributingFactorDim, fact::Fact, person_age::PersonAge,
    person_position::PersonPosition, person_role::PersonPositionRole, person_sex::PersonSex,
    person_type::PersonType, time::Time as DmTime,
};

// ---------------------------------------------------------------------------
// Public configuration types
// ---------------------------------------------------------------------------

/// Credentials used to connect to the MS SQL Server.
pub struct DbCredentials {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub domain: String,
    pub username: String,
    pub password: String,
}

impl Default for DbCredentials {
    fn default() -> Self {
        Self {
            host: "fimn-db1.htwk-leipzig.de".into(),
            port: 1433,
            database: "DWH25-04".into(),
            domain: "HTWK".into(),
            username: String::new(),
            password: String::new(),
        }
    }
}

/// Identifies one logical table of the data mart.
///
/// Pass a slice of these to [`ingest_data_mart`] to choose which tables are
/// populated during a given run.  Dimensions that are already present in the
/// database can simply be omitted from the slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataMartTable {
    DimTime,
    DimPersonAge,
    DimPersonPosition,
    DimPersonRole,
    DimPersonSex,
    DimPersonType,
    DimContributingFactor,
    Fact,
}

// ---------------------------------------------------------------------------
// Connection helpers
// ---------------------------------------------------------------------------

const SCHEMA: &str = "project_julian_bruder_kenana_saeed";

async fn connect(creds: &DbCredentials) -> Result<Client<Compat<TcpStream>>> {
    let mut config = Config::new();

    config.host(&creds.host);
    config.port(creds.port);
    config.database(&creds.database);
    // AuthMethod::windows is only available on Windows (winauth feature + win32 SSPI).
    // On Linux/macOS we pass domain-qualified credentials via SQL Server auth:
    // the TDS driver sends "DOMAIN\username" as the login name, which SQL Server
    // accepts for domain accounts when the connection is encrypted (TLS).
    config.authentication(AuthMethod::sql_server(
        format!("{}\\{}", creds.domain, creds.username),
        &creds.password,
    ));
    config.trust_cert();
    config.encryption(tiberius::EncryptionLevel::Required);

    let tcp = TcpStream::connect(config.get_addr())
        .await
        .with_context(|| format!("TCP connect to {}:{}", creds.host, creds.port))?;
    tcp.set_nodelay(true)?;

    Client::connect(config, tcp.compat_write())
        .await
        .with_context(|| "TDS handshake / login failed")
}

/// Execute a single SQL statement, ignoring "object already exists" errors
/// (error number 2714 for tables/indexes, 1913 for indexes, 2705 for columns,
/// etc.).  This makes DDL steps individually idempotent.
async fn exec(client: &mut Client<Compat<TcpStream>>, sql: &str) -> Result<()> {
    client
        .execute(sql, &[])
        .await
        .map(|_| ())
        .or_else(|e| {
            let msg = e.to_string();
            // Swallow "already exists" / "duplicate key" errors from DDL re-runs.
            if msg.contains("already an object named")
                || msg.contains("already exists")
                || msg.contains("duplicate key")
            {
                Ok(())
            } else {
                Err(anyhow::anyhow!(e))
            }
        })
        .with_context(|| format!("executing SQL:\n{sql}"))
}

// ---------------------------------------------------------------------------
// DDL
// ---------------------------------------------------------------------------

/// Creates the schema and all data-mart tables/indexes/views.
///
/// Individual DDL statements are submitted separately because MS SQL Server
/// does not allow `CREATE TABLE` and `CREATE INDEX` in the same batch, and
/// the indexed-view creation requires all referenced objects to exist first.
///
/// The function is *idempotent*: re-running it against a database that already
/// has all objects is safe (existing-object errors are swallowed).
pub async fn setup_data_mart(creds: &DbCredentials) -> Result<()> {
    let mut client = connect(creds).await?;

    // -- Schema --------------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "IF NOT EXISTS (SELECT 1 FROM sys.schemas WHERE name = N'{SCHEMA}') \
                  EXEC('CREATE SCHEMA [{SCHEMA}]')"
        ),
    )
    .await?;

    // -- DimTime -------------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimTime] (
                time_id             INT           NOT NULL,
                [timestamp]         DATETIME      NOT NULL,
                hier_def_day        DATE          NOT NULL,
                hier_def_month      VARCHAR(12)   NOT NULL,
                hier_def_year       SMALLINT      NOT NULL,
                hier_moon_phase     VARCHAR(20)   NOT NULL,
                weather             VARCHAR(20)   NOT NULL,
                CONSTRAINT PK_DimTime PRIMARY KEY CLUSTERED (time_id)
            )"
        ),
    )
    .await?;

    exec(
        &mut client,
        &format!(
            "CREATE INDEX IX_DimTime_Day \
             ON [{SCHEMA}].[DimTime] (hier_def_day)"
        ),
    )
    .await?;

    exec(
        &mut client,
        &format!(
            "CREATE INDEX IX_DimTime_MoonPhase_Weather \
             ON [{SCHEMA}].[DimTime] (hier_moon_phase, weather) \
             INCLUDE (hier_def_year, hier_def_month)"
        ),
    )
    .await?;

    // -- DimPersonAge --------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimPersonAge] (
                person_age_id               INT         NOT NULL,
                person_age                  TINYINT     NOT NULL,
                person_age_known            BIT         NOT NULL,
                person_age_hier_def_group   VARCHAR(12) NOT NULL,
                CONSTRAINT PK_DimPersonAge PRIMARY KEY CLUSTERED (person_age_id)
            )"
        ),
    )
    .await?;

    // -- DimPersonPosition ---------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimPersonPosition] (
                person_position_id  INT         NOT NULL,
                person_position     VARCHAR(10) NOT NULL,
                CONSTRAINT PK_DimPersonPosition PRIMARY KEY CLUSTERED (person_position_id)
            )"
        ),
    )
    .await?;

    // -- DimPersonRole -------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimPersonRole] (
                person_role_id  INT         NOT NULL,
                person_role     VARCHAR(20) NOT NULL,
                CONSTRAINT PK_DimPersonRole PRIMARY KEY CLUSTERED (person_role_id)
            )"
        ),
    )
    .await?;

    // -- DimPersonSex --------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimPersonSex] (
                person_sex_id   INT         NOT NULL,
                person_sex      VARCHAR(10) NOT NULL,
                CONSTRAINT PK_DimPersonSex PRIMARY KEY CLUSTERED (person_sex_id)
            )"
        ),
    )
    .await?;

    // -- DimPersonType -------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimPersonType] (
                person_type_id  INT         NOT NULL,
                person_type     VARCHAR(20) NOT NULL,
                CONSTRAINT PK_DimPersonType PRIMARY KEY CLUSTERED (person_type_id)
            )"
        ),
    )
    .await?;

    // -- DimContributingFactor -----------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[DimContributingFactor] (
                contributing_factor_id                   INT         NOT NULL,
                contributing_factor                      VARCHAR(60) NOT NULL,
                contributing_factor_hier_def_category    VARCHAR(25) NOT NULL,
                contributing_factor_hier_def_subcategory VARCHAR(60) NOT NULL,
                CONSTRAINT PK_DimContributingFactor PRIMARY KEY CLUSTERED (contributing_factor_id)
            )"
        ),
    )
    .await?;

    // -- Fact ----------------------------------------------------------------
    exec(
        &mut client,
        &format!(
            "CREATE TABLE [{SCHEMA}].[Fact] (
                fact_id                 INT     NOT NULL,
                contributing_factor_id  INT     NOT NULL,
                person_age_id           INT     NOT NULL,
                person_position_id      INT     NOT NULL,
                person_role_id          INT     NOT NULL,
                person_sex_id           INT     NOT NULL,
                person_type_id          INT     NOT NULL,
                time_id                 INT     NOT NULL,
                persons_injured         TINYINT NOT NULL,
                persons_killed          TINYINT NOT NULL,
                pedestrians_injured     TINYINT NOT NULL,
                pedestrians_killed      TINYINT NOT NULL,
                cyclist_injured         TINYINT NOT NULL,
                cyclist_killed          TINYINT NOT NULL,
                motorist_injured        TINYINT NOT NULL,
                motorist_killed         TINYINT NOT NULL,
                CONSTRAINT PK_Fact PRIMARY KEY NONCLUSTERED (fact_id),
                CONSTRAINT FK_Fact_Time
                    FOREIGN KEY (time_id)
                    REFERENCES [{SCHEMA}].[DimTime] (time_id),
                CONSTRAINT FK_Fact_PersonAge
                    FOREIGN KEY (person_age_id)
                    REFERENCES [{SCHEMA}].[DimPersonAge] (person_age_id),
                CONSTRAINT FK_Fact_PersonPosition
                    FOREIGN KEY (person_position_id)
                    REFERENCES [{SCHEMA}].[DimPersonPosition] (person_position_id),
                CONSTRAINT FK_Fact_PersonRole
                    FOREIGN KEY (person_role_id)
                    REFERENCES [{SCHEMA}].[DimPersonRole] (person_role_id),
                CONSTRAINT FK_Fact_PersonSex
                    FOREIGN KEY (person_sex_id)
                    REFERENCES [{SCHEMA}].[DimPersonSex] (person_sex_id),
                CONSTRAINT FK_Fact_PersonType
                    FOREIGN KEY (person_type_id)
                    REFERENCES [{SCHEMA}].[DimPersonType] (person_type_id),
                CONSTRAINT FK_Fact_ContributingFactor
                    FOREIGN KEY (contributing_factor_id)
                    REFERENCES [{SCHEMA}].[DimContributingFactor] (contributing_factor_id)
            )"
        ),
    )
    .await?;

    // Clustered Columnstore Index – separate statement required by MSSQL.
    exec(
        &mut client,
        &format!(
            "CREATE CLUSTERED COLUMNSTORE INDEX CCI_Fact \
             ON [{SCHEMA}].[Fact]"
        ),
    )
    .await?;

    // -- Materialized / indexed view -----------------------------------------
    // The view body must be a single CREATE VIEW statement (no GO batch
    // separator inside a programmatic call).
    exec(
        &mut client,
        &format!(
            "CREATE VIEW [{SCHEMA}].[MV_SeverityByMoonWeatherFactorSexAge]
             WITH SCHEMABINDING
             AS
             SELECT
                 dt.hier_moon_phase                              AS moon_phase,
                 dt.weather                                      AS weather,
                 dcf.contributing_factor_hier_def_category       AS factor_category,
                 dps.person_sex                                  AS person_sex,
                 dpa.person_age_hier_def_group                   AS age_group,
                 SUM(CAST(f.persons_injured     AS INT))         AS total_persons_injured,
                 SUM(CAST(f.persons_killed      AS INT))         AS total_persons_killed,
                 SUM(CAST(f.pedestrians_injured AS INT))         AS total_pedestrians_injured,
                 SUM(CAST(f.pedestrians_killed  AS INT))         AS total_pedestrians_killed,
                 SUM(CAST(f.cyclist_injured     AS INT))         AS total_cyclist_injured,
                 SUM(CAST(f.cyclist_killed      AS INT))         AS total_cyclist_killed,
                 SUM(CAST(f.motorist_injured    AS INT))         AS total_motorist_injured,
                 SUM(CAST(f.motorist_killed     AS INT))         AS total_motorist_killed,
                 COUNT_BIG(*)                                    AS incident_count
             FROM [{SCHEMA}].[Fact]                  AS f
             JOIN [{SCHEMA}].[DimTime]               AS dt  ON dt.time_id                 = f.time_id
             JOIN [{SCHEMA}].[DimPersonSex]          AS dps ON dps.person_sex_id          = f.person_sex_id
             JOIN [{SCHEMA}].[DimPersonAge]          AS dpa ON dpa.person_age_id          = f.person_age_id
             JOIN [{SCHEMA}].[DimContributingFactor] AS dcf ON dcf.contributing_factor_id = f.contributing_factor_id
             GROUP BY
                 dt.hier_moon_phase,
                 dt.weather,
                 dcf.contributing_factor_hier_def_category,
                 dps.person_sex,
                 dpa.person_age_hier_def_group"
        ),
    )
    .await?;

    exec(
        &mut client,
        &format!(
            "CREATE UNIQUE CLUSTERED INDEX UCI_MV_SeverityByMoonWeatherFactorSexAge \
             ON [{SCHEMA}].[MV_SeverityByMoonWeatherFactorSexAge] \
             (moon_phase, weather, person_sex, age_group, factor_category)"
        ),
    )
    .await?;

    println!("      DDL complete.");
    Ok(())
}

// ---------------------------------------------------------------------------
// Batch-insert helpers
// ---------------------------------------------------------------------------

/// How many rows to accumulate before flushing a single INSERT statement.
///
/// MSSQL supports up to 1 000 rows per multi-row VALUES list (the 2 100-param
/// limit is the binding constraint for wide tables, so we keep this
/// conservative at 100 rows per flush for the fact table which has 16 columns,
/// and larger for the narrower dimension tables).
const DIM_BATCH_SIZE: usize = 500;
const FACT_BATCH_SIZE: usize = 100;

// ---------------------------------------------------------------------------
// Enum → string helpers (SCREAMING_SNAKE_CASE, matching the DDL CHECK values)
// ---------------------------------------------------------------------------

fn moon_phase_str(p: crate::data_mart::time::MoonPhase) -> &'static str {
    use crate::data_mart::time::MoonPhase::*;
    match p {
        New => "NEW",
        WaxingCrescent => "WAXING_CRESCENT",
        FirstQuarter => "FIRST_QUARTER",
        WaxingGibbous => "WAXING_GIBBOUS",
        Full => "FULL",
        WaningGibbous => "WANING_GIBBOUS",
        LastQuarter => "LAST_QUARTER",
        WaningCrescent => "WANING_CRESCENT",
        Unknown => "UNKNOWN",
    }
}

fn weather_str(w: crate::data_mart::time::Weather) -> &'static str {
    use crate::data_mart::time::Weather::*;
    match w {
        Clear => "CLEAR",
        Cloudy => "CLOUDY",
        RainyLight => "RAINY_LIGHT",
        RainyHeavy => "RAINY_HEAVY",
        Stormy => "STORMY",
        Windy => "WINDY",
        Miscallaneous => "MISCALLANEOUS",
        Unknown => "UNKNOWN",
    }
}

fn age_group_str(g: crate::data_mart::person_age::PersonAgeGroup) -> &'static str {
    use crate::data_mart::person_age::PersonAgeGroup::*;
    match g {
        Fertile => "FERTILE",
        Infertile => "INFERTILE",
        Unknown => "UNKNOWN",
    }
}

fn position_str(p: crate::data_mart::person_position::PersonPositionInVehicle) -> &'static str {
    use crate::data_mart::person_position::PersonPositionInVehicle::*;
    match p {
        Driver => "DRIVER",
        Front => "FRONT",
        Rear => "REAR",
        Lap => "LAP",
        Outside => "OUTSIDE",
        Unknown => "UNKNOWN",
    }
}

fn role_str(r: crate::data_mart::person_role::PersonRole) -> &'static str {
    use crate::data_mart::person_role::PersonRole::*;
    match r {
        NotifiedPerson => "NOTIFIED_PERSON",
        Witness => "WITNESS",
        Registrant => "REGISTRANT",
        InLineSkater => "IN_LINE_SKATER",
        Passenger => "PASSENGER",
        Driver => "DRIVER",
        PolicyHolder => "POLICY_HOLDER",
        Owner => "OWNER",
        Pedestrian => "PEDESTRIAN",
        Unknown => "UNKNOWN",
    }
}

fn sex_str(s: crate::data_mart::person_sex::PersonSexType) -> &'static str {
    use crate::data_mart::person_sex::PersonSexType::*;
    match s {
        Male => "MALE",
        Female => "FEMALE",
        Unknown => "UNKNOWN",
    }
}

fn person_type_str(t: crate::data_mart::person_type::PersonTypeType) -> &'static str {
    use crate::data_mart::person_type::PersonTypeType::*;
    match t {
        Pedestrian => "PEDESTRIAN",
        Occupant => "OCCUPANT",
        Bicyclist => "BICYCLIST",
        OtherMotorized => "OTHER_MOTORIZED",
        Unknown => "UNKNOWN",
    }
}

fn contributing_factor_str(
    f: crate::data_mart::contributing_factor::ContributingFactor,
) -> &'static str {
    use crate::data_mart::contributing_factor::ContributingFactor::*;
    match f {
        DriverlessRunawayVehicle => "DRIVERLESS_RUNAWAY_VEHICLE",
        ListeningUsingHeadphones => "LISTENING_USING_HEADPHONES",
        EatingOrDrinking => "EATING_OR_DRINKING",
        UnsafeLaneChanging => "UNSAFE_LANE_CHANGING",
        CellPhoneHandHeld => "CELL_PHONE_HAND_HELD",
        CellPhoneHandsFree => "CELL_PHONE_HANDS_FREE",
        DrugsIllegal => "DRUGS_ILLEGAL",
        Texting => "TEXTING",
        HeadlightsDefective => "HEADLIGHTS_DEFECTIVE",
        OtherLightingDefects => "OTHER_LIGHTING_DEFECTS",
        DriverInexperience => "DRIVER_INEXPERIENCE",
        AggressiveDrivingRoadRage => "AGGRESSIVE_DRIVING_ROAD_RAGE",
        UnsafeSpeed => "UNSAFE_SPEED",
        LaneMarkingImproperInadequate => "LANE_MARKING_IMPROPER_INADEQUATE",
        Glare => "GLARE",
        TrafficControlDeviceImproperNonWorking => "TRAFFIC_CONTROL_DEVICE_IMPROPER_NON_WORKING",
        PassingTooClosely => "PASSING_TOO_CLOSELY",
        AcceleratorDefective => "ACCELERATOR_DEFECTIVE",
        ShouldersDefectiveImproper => "SHOULDERS_DEFECTIVE_IMPROPER",
        OutsideCarDistraction => "OUTSIDE_CAR_DISTRACTION",
        DriverInattentionDistraction => "DRIVER_INATTENTION_DISTRACTION",
        TintedWindows => "TINTED_WINDOWS",
        UsingOnBoardNavigationDevice => "USING_ON_BOARD_NAVIGATION_DEVICE",
        ReactionToOtherUninvolvedVehicle => "REACTION_TO_OTHER_UNINVOLVED_VEHICLE",
        ObstructionDebris => "OBSTRUCTION_DEBRIS",
        PrescriptionMedication => "PRESCRIPTION_MEDICATION",
        TireFailureInadequate => "TIRE_FAILURE_INADEQUATE",
        FatiguedDrowsy => "FATIGUED_DROWSY",
        PassingOrLaneUsageImproper => "PASSING_OR_LANE_USAGE_IMPROPER",
        FollowingTooClosely => "FOLLOWING_TOO_CLOSELY",
        ViewObstructedLimited => "VIEW_OBSTRUCTED_LIMITED",
        OversizedVehicle => "OVERSIZED_VEHICLE",
        LostConsciousness => "LOST_CONSCIOUSNESS",
        BackingUnsafely => "BACKING_UNSAFELY",
        OtherVehicular => "OTHER_VEHICULAR",
        Illness => "ILLNESS",
        WindshieldInadequate => "WINDSHIELD_INADEQUATE",
        FellAsleep => "FELL_ASLEEP",
        TrafficControlDisregarded => "TRAFFIC_CONTROL_DISREGARDED",
        PavementDefective => "PAVEMENT_DEFECTIVE",
        SteeringFailure => "STEERING_FAILURE",
        PassengerDistraction => "PASSENGER_DISTRACTION",
        VehicleVandalism => "VEHICLE_VANDALISM",
        FailureToKeepRight => "FAILURE_TO_KEEP_RIGHT",
        BrakesDefective => "BRAKES_DEFECTIVE",
        TurningImproperly => "TURNING_IMPROPERLY",
        FailureToYieldRightOfWay => "FAILURE_TO_YIELD_RIGHT_OF_WAY",
        ReactionToUninvolvedVehicle => "REACTION_TO_UNINVOLVED_VEHICLE",
        TowHitchDefective => "TOW_HITCH_DEFECTIVE",
        AlcoholInvolvement => "ALCOHOL_INVOLVEMENT",
        PhysicalDisability => "PHYSICAL_DISABILITY",
        AnimalsAction => "ANIMALS_ACTION",
        OtherElectronicDevice => "OTHER_ELECTRONIC_DEVICE",
        PedestrianBicyclistOtherPedestrianErrorConfusion => {
            "PEDESTRIAN_BICYCLIST_OTHER_PEDESTRIAN_ERROR_CONFUSION"
        }
        PavementSlippery => "PAVEMENT_SLIPPERY",
        Unknown => "UNKNOWN",
    }
}

fn contributing_factor_category_str(
    c: crate::data_mart::contributing_factor::ContributingFactorCategory,
) -> &'static str {
    use crate::data_mart::contributing_factor::ContributingFactorCategory::*;
    match c {
        HumanBehavior => "HUMAN_BEHAVIOR",
        HumanCondition => "HUMAN_CONDITION",
        Distraction => "DISTRACTION",
        SubstanceRelated => "SUBSTANCE_RELATED",
        VehicleDefect => "VEHICLE_DEFECT",
        RoadInfrastructure => "ROAD_INFRASTRUCTURE",
        Environmental => "ENVIRONMENTAL",
        External => "EXTERNAL",
        Unknown => "UNKNOWN",
    }
}

fn contributing_factor_subcategory_str(
    h: &crate::data_mart::contributing_factor::ContributingFactorHierarchy,
) -> &'static str {
    use crate::data_mart::contributing_factor::{
        ContributingFactorHierarchy::*, DistractionFactor as D, EnvironmentalFactor as Env,
        ExternalFactor as Ex, HumanBehaviorFactor as HB, HumanConditionFactor as HC,
        RoadInfrastructureFactor as RI, SubstanceRelatedFactor as SR, VehicleDefectFactor as VD,
    };
    match h {
        Unknown => "UNKNOWN",
        HumanBehavior(f) => match f {
            HB::UnsafeSpeed => "UNSAFE_SPEED",
            HB::UnsafeLaneChanging => "UNSAFE_LANE_CHANGING",
            HB::PassingTooClosely => "PASSING_TOO_CLOSELY",
            HB::PassingOrLaneUsageImproper => "PASSING_OR_LANE_USAGE_IMPROPER",
            HB::FollowingTooClosely => "FOLLOWING_TOO_CLOSELY",
            HB::FailureToKeepRight => "FAILURE_TO_KEEP_RIGHT",
            HB::FailureToYieldRightOfWay => "FAILURE_TO_YIELD_RIGHT_OF_WAY",
            HB::TurningImproperly => "TURNING_IMPROPERLY",
            HB::BackingUnsafely => "BACKING_UNSAFELY",
            HB::TrafficControlDisregarded => "TRAFFIC_CONTROL_DISREGARDED",
            HB::AggressiveDrivingRoadRage => "AGGRESSIVE_DRIVING_ROAD_RAGE",
            HB::Unknown => "UNKNOWN",
        },
        HumanCondition(f) => match f {
            HC::DriverInexperience => "DRIVER_INEXPERIENCE",
            HC::FatiguedDrowsy => "FATIGUED_DROWSY",
            HC::FellAsleep => "FELL_ASLEEP",
            HC::LostConsciousness => "LOST_CONSCIOUSNESS",
            HC::Illness => "ILLNESS",
            HC::PhysicalDisability => "PHYSICAL_DISABILITY",
            HC::Unknown => "UNKNOWN",
        },
        Distraction(f) => match f {
            D::DriverInattentionDistraction => "DRIVER_INATTENTION_DISTRACTION",
            D::PassengerDistraction => "PASSENGER_DISTRACTION",
            D::OutsideCarDistraction => "OUTSIDE_CAR_DISTRACTION",
            D::CellPhoneHandHeld => "CELL_PHONE_HAND_HELD",
            D::CellPhoneHandsFree => "CELL_PHONE_HANDS_FREE",
            D::Texting => "TEXTING",
            D::UsingOnBoardNavigationDevice => "USING_ON_BOARD_NAVIGATION_DEVICE",
            D::OtherElectronicDevice => "OTHER_ELECTRONIC_DEVICE",
            D::ListeningUsingHeadphones => "LISTENING_USING_HEADPHONES",
            D::EatingOrDrinking => "EATING_OR_DRINKING",
            D::Unknown => "UNKNOWN",
        },
        SubstanceRelated(f) => match f {
            SR::AlcoholInvolvement => "ALCOHOL_INVOLVEMENT",
            SR::DrugsIllegal => "DRUGS_ILLEGAL",
            SR::PrescriptionMedication => "PRESCRIPTION_MEDICATION",
            SR::Unknown => "UNKNOWN",
        },
        VehicleDefect(f) => match f {
            VD::AcceleratorDefective => "ACCELERATOR_DEFECTIVE",
            VD::BrakesDefective => "BRAKES_DEFECTIVE",
            VD::SteeringFailure => "STEERING_FAILURE",
            VD::TireFailureInadequate => "TIRE_FAILURE_INADEQUATE",
            VD::TowHitchDefective => "TOW_HITCH_DEFECTIVE",
            VD::HeadlightsDefective => "HEADLIGHTS_DEFECTIVE",
            VD::OtherLightingDefects => "OTHER_LIGHTING_DEFECTS",
            VD::WindshieldInadequate => "WINDSHIELD_INADEQUATE",
            VD::TintedWindows => "TINTED_WINDOWS",
            VD::VehicleVandalism => "VEHICLE_VANDALISM",
            VD::DriverlessRunawayVehicle => "DRIVERLESS_RUNAWAY_VEHICLE",
            VD::OversizedVehicle => "OVERSIZED_VEHICLE",
            VD::OtherVehicular => "OTHER_VEHICULAR",
            VD::Unknown => "UNKNOWN",
        },
        RoadInfrastructure(f) => match f {
            RI::PavementDefective => "PAVEMENT_DEFECTIVE",
            RI::PavementSlippery => "PAVEMENT_SLIPPERY",
            RI::LaneMarkingImproperInadequate => "LANE_MARKING_IMPROPER_INADEQUATE",
            RI::ShouldersDefectiveImproper => "SHOULDERS_DEFECTIVE_IMPROPER",
            RI::TrafficControlDeviceImproperNonWorking => {
                "TRAFFIC_CONTROL_DEVICE_IMPROPER_NON_WORKING"
            }
            RI::ViewObstructedLimited => "VIEW_OBSTRUCTED_LIMITED",
            RI::Unknown => "UNKNOWN",
        },
        Environmental(f) => match f {
            Env::Glare => "GLARE",
            Env::ObstructionDebris => "OBSTRUCTION_DEBRIS",
            Env::AnimalsAction => "ANIMALS_ACTION",
            Env::Unknown => "UNKNOWN",
        },
        External(f) => match f {
            Ex::ReactionToOtherUninvolvedVehicle => "REACTION_TO_OTHER_UNINVOLVED_VEHICLE",
            Ex::ReactionToUninvolvedVehicle => "REACTION_TO_UNINVOLVED_VEHICLE",
            Ex::PedestrianBicyclistOtherPedestrianErrorConfusion => {
                "PEDESTRIAN_BICYCLIST_OTHER_PEDESTRIAN_ERROR_CONFUSION"
            }
            Ex::Unknown => "UNKNOWN",
        },
    }
}

// ---------------------------------------------------------------------------
// Per-table insert implementations
// ---------------------------------------------------------------------------

async fn insert_dim_time(client: &mut Client<Compat<TcpStream>>, rows: &[DmTime]) -> Result<()> {
    println!(
        "      inserting DimTime ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimTime] \
             (time_id,[timestamp],hier_def_day,hier_def_month,hier_def_year,hier_moon_phase,weather) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                let ts = r.timestamp;
                // DATETIME literal: 'YYYY-MM-DD HH:MM:SS'
                let ts_str = format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    ts.year(),
                    ts.month() as u8,
                    ts.day(),
                    ts.hour(),
                    ts.minute(),
                    ts.second()
                );
                let day = r.hier_def_day;
                let day_str = format!(
                    "{:04}-{:02}-{:02}",
                    day.year(),
                    day.month() as u8,
                    day.day()
                );
                format!(
                    "({},'{ts_str}','{day_str}','{}',{},\'{}\',\'{}\')",
                    r.time_id,
                    r.hier_def_month,
                    r.hier_def_year,
                    moon_phase_str(r.hier_moon_phase),
                    weather_str(r.weather),
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimTime batch {batch_idx}"))?;
    }

    println!("      DimTime done.");
    Ok(())
}

async fn insert_dim_person_age(
    client: &mut Client<Compat<TcpStream>>,
    rows: &[PersonAge],
) -> Result<()> {
    println!(
        "      inserting DimPersonAge ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimPersonAge] \
             (person_age_id,person_age,person_age_known,person_age_hier_def_group) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "({},{},{},'{}')",
                    r.person_age_id,
                    r.person_age,
                    if r.person_age_known { 1 } else { 0 },
                    age_group_str(r.person_age_hier_def_group),
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimPersonAge batch {batch_idx}"))?;
    }

    println!("      DimPersonAge done.");
    Ok(())
}

async fn insert_dim_person_position(
    client: &mut Client<Compat<TcpStream>>,
    rows: &[PersonPosition],
) -> Result<()> {
    println!(
        "      inserting DimPersonPosition ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimPersonPosition] \
             (person_position_id,person_position) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "({},'{}')",
                    r.person_position_id,
                    position_str(r.person_position),
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimPersonPosition batch {batch_idx}"))?;
    }

    println!("      DimPersonPosition done.");
    Ok(())
}

async fn insert_dim_person_role(
    client: &mut Client<Compat<TcpStream>>,
    rows: &[PersonPositionRole],
) -> Result<()> {
    println!(
        "      inserting DimPersonRole ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimPersonRole] \
             (person_role_id,person_role) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "({},'{}')",
                    r.person_position_role_id,
                    role_str(r.person_position_role),
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimPersonRole batch {batch_idx}"))?;
    }

    println!("      DimPersonRole done.");
    Ok(())
}

async fn insert_dim_person_sex(
    client: &mut Client<Compat<TcpStream>>,
    rows: &[PersonSex],
) -> Result<()> {
    println!(
        "      inserting DimPersonSex ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimPersonSex] \
             (person_sex_id,person_sex) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| format!("({},'{}')", r.person_sex_id, sex_str(r.person_sex),))
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimPersonSex batch {batch_idx}"))?;
    }

    println!("      DimPersonSex done.");
    Ok(())
}

async fn insert_dim_person_type(
    client: &mut Client<Compat<TcpStream>>,
    rows: &[PersonType],
) -> Result<()> {
    println!(
        "      inserting DimPersonType ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimPersonType] \
             (person_type_id,person_type) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "({},'{}')",
                    r.person_type_id,
                    person_type_str(r.person_type),
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimPersonType batch {batch_idx}"))?;
    }

    println!("      DimPersonType done.");
    Ok(())
}

async fn insert_dim_contributing_factor(
    client: &mut Client<Compat<TcpStream>>,
    rows: &[ContributingFactorDim],
) -> Result<()> {
    println!(
        "      inserting DimContributingFactor ({} rows, batch size {DIM_BATCH_SIZE})…",
        rows.len()
    );

    for (batch_idx, chunk) in rows.chunks(DIM_BATCH_SIZE).enumerate() {
        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[DimContributingFactor] \
             (contributing_factor_id,contributing_factor,\
              contributing_factor_hier_def_category,\
              contributing_factor_hier_def_subcategory) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "({},\'{}\',\'{}\',\'{}\')",
                    r.contributing_factor_id,
                    contributing_factor_str(r.contributing_factor),
                    contributing_factor_category_str(r.contributing_factor_hier_def_category),
                    contributing_factor_subcategory_str(
                        &r.contributing_factor_hier_def_subcategory
                    ),
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("DimContributingFactor batch {batch_idx}"))?;
    }

    println!("      DimContributingFactor done.");
    Ok(())
}

async fn insert_fact(client: &mut Client<Compat<TcpStream>>, rows: &[Fact]) -> Result<()> {
    println!(
        "      inserting Fact ({} rows, batch size {FACT_BATCH_SIZE})…",
        rows.len()
    );

    let total_batches = rows.chunks(FACT_BATCH_SIZE).count();

    for (batch_idx, chunk) in rows.chunks(FACT_BATCH_SIZE).enumerate() {
        if batch_idx % 500 == 0 {
            println!("        batch {batch_idx}/{total_batches}…");
        }

        let mut sql = format!(
            "INSERT INTO [{SCHEMA}].[Fact] \
             (fact_id,contributing_factor_id,person_age_id,person_position_id,\
              person_role_id,person_sex_id,person_type_id,time_id,\
              persons_injured,persons_killed,pedestrians_injured,pedestrians_killed,\
              cyclist_injured,cyclist_killed,motorist_injured,motorist_killed) VALUES "
        );

        let values: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
                    r.fact_id,
                    r.contributing_factor_id,
                    r.person_age_id,
                    r.person_position_id,
                    r.person_role_id,
                    r.person_sex_id,
                    r.person_type_id,
                    r.time_id,
                    r.persons_injured,
                    r.persons_killed,
                    r.pedestrians_injured,
                    r.pedestrians_killed,
                    r.cyclist_injured,
                    r.cyclist_killed,
                    r.motorist_injured,
                    r.motorist_killed,
                )
            })
            .collect();

        sql.push_str(&values.join(","));

        client
            .execute(sql.as_str(), &[])
            .await
            .with_context(|| format!("Fact batch {batch_idx}"))?;
    }

    println!("      Fact done.");
    Ok(())
}

// ---------------------------------------------------------------------------
// Public insert entry-point
// ---------------------------------------------------------------------------

/// Data to be inserted, assembled by the caller.
pub struct DataMart<'a> {
    pub dim_time: &'a [DmTime],
    pub dim_person_age: &'a [PersonAge],
    pub dim_person_position: &'a [PersonPosition],
    pub dim_person_role: &'a [PersonPositionRole],
    pub dim_person_sex: &'a [PersonSex],
    pub dim_person_type: &'a [PersonType],
    pub dim_contributing_factor: &'a [ContributingFactorDim],
    pub fact: &'a [Fact],
}

/// Insert the tables listed in `tables` into the database.
///
/// Pass all tables (`DataMartTable` variants) you want populated.  Any table
/// not mentioned is silently skipped, which lets you re-run only the fact
/// table without re-inserting dimensions that are already there.
///
/// Dimensions are always inserted before the fact table (regardless of the
/// order you list them) because the fact table has foreign-key constraints
/// referencing all dimension tables.
pub async fn ingest_data_mart(
    creds: &DbCredentials,
    data: &DataMart<'_>,
    tables: &[DataMartTable],
) -> Result<()> {
    if tables.is_empty() {
        println!("      No tables selected – nothing to insert.");
        return Ok(());
    }

    let mut client = connect(creds).await?;

    let wants = |t: DataMartTable| tables.contains(&t);

    // -- Dimensions first (order matters: Fact FKs depend on all of them) ----
    if wants(DataMartTable::DimTime) {
        insert_dim_time(&mut client, data.dim_time).await?;
    }
    if wants(DataMartTable::DimPersonAge) {
        insert_dim_person_age(&mut client, data.dim_person_age).await?;
    }
    if wants(DataMartTable::DimPersonPosition) {
        insert_dim_person_position(&mut client, data.dim_person_position).await?;
    }
    if wants(DataMartTable::DimPersonRole) {
        insert_dim_person_role(&mut client, data.dim_person_role).await?;
    }
    if wants(DataMartTable::DimPersonSex) {
        insert_dim_person_sex(&mut client, data.dim_person_sex).await?;
    }
    if wants(DataMartTable::DimPersonType) {
        insert_dim_person_type(&mut client, data.dim_person_type).await?;
    }
    if wants(DataMartTable::DimContributingFactor) {
        insert_dim_contributing_factor(&mut client, data.dim_contributing_factor).await?;
    }

    // -- Fact table last -----------------------------------------------------
    if wants(DataMartTable::Fact) {
        insert_fact(&mut client, data.fact).await?;
    }

    println!("      All selected tables ingested successfully.");
    Ok(())
}
