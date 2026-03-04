-- =============================================================================
-- Data Mart DDL  –  NYC Vehicle Incidents Star Schema
-- =============================================================================
-- Naming conventions:
--   Dimension tables : dm.<DimName>
--   Fact table       : dm.Fact
-- All surrogate keys are INT; 0 is reserved as the "Unknown" sentinel.
-- =============================================================================


-- =============================================================================
-- Dimension: Time
-- Parallel hierarchies:
--   Default  : timestamp → day → month → year
--   Moon     : timestamp → moon_phase
-- Denormalized weather attribute stored directly on the dimension row.
-- =============================================================================
CREATE TABLE dm.DimTime (
    time_id                 INT              NOT NULL,
    [timestamp]             DATETIMEOFFSET(0) NOT NULL,

    -- Default hierarchy
    hier_def_day            DATE             NOT NULL,
    hier_def_month          VARCHAR(12)      NOT NULL,   -- e.g. 'January'
    hier_def_year           SMALLINT         NOT NULL,

    -- Moon-phase hierarchy
    hier_moon_phase         VARCHAR(20)      NOT NULL,

    -- Denormalized weather attribute
    weather                 VARCHAR(20)      NOT NULL,

    CONSTRAINT PK_DimTime PRIMARY KEY CLUSTERED (time_id),

    CONSTRAINT CK_DimTime_MoonPhase CHECK (hier_moon_phase IN (
        'NEW',
        'WAXING_CRESCENT',
        'FIRST_QUARTER',
        'WAXING_GIBBOUS',
        'FULL',
        'WANING_GIBBOUS',
        'LAST_QUARTER',
        'WANING_CRESCENT',
        'UNKNOWN'
    )),

    CONSTRAINT CK_DimTime_Weather CHECK (weather IN (
        'CLEAR',
        'CLOUDY',
        'RAINY_LIGHT',
        'RAINY_HEAVY',
        'STORMY',
        'WINDY',
        'MISCALLANEOUS',
        'UNKNOWN'
    )),

    CONSTRAINT CK_DimTime_Month CHECK (hier_def_month IN (
        'January', 'February', 'March', 'April', 'May', 'June',
        'July', 'August', 'September', 'October', 'November', 'December'
    ))
);

CREATE INDEX IX_DimTime_Timestamp  ON dm.DimTime ([timestamp]);
CREATE INDEX IX_DimTime_Day        ON dm.DimTime (hier_def_day);
CREATE INDEX IX_DimTime_Year       ON dm.DimTime (hier_def_year);
CREATE INDEX IX_DimTime_MoonPhase  ON dm.DimTime (hier_moon_phase);
CREATE INDEX IX_DimTime_Weather    ON dm.DimTime (weather);


-- =============================================================================
-- Dimension: Person Age
-- Hierarchy: age → age_group (Fertile / Infertile / Unknown)
-- Row 0 is the "age unknown" sentinel.
-- =============================================================================
CREATE TABLE dm.DimPersonAge (
    person_age_id               INT         NOT NULL,
    person_age                  TINYINT     NOT NULL,   -- 0 means "not known"
    person_age_known            BIT         NOT NULL,
    person_age_hier_def_group   VARCHAR(12) NOT NULL,

    CONSTRAINT PK_DimPersonAge PRIMARY KEY CLUSTERED (person_age_id),

    CONSTRAINT CK_DimPersonAge_Group CHECK (person_age_hier_def_group IN (
        'FERTILE',
        'INFERTILE',
        'UNKNOWN'
    ))
);


-- =============================================================================
-- Dimension: Person Position in Vehicle
-- =============================================================================
CREATE TABLE dm.DimPersonPosition (
    person_position_id      INT         NOT NULL,
    person_position         VARCHAR(10) NOT NULL,

    CONSTRAINT PK_DimPersonPosition PRIMARY KEY CLUSTERED (person_position_id),

    CONSTRAINT CK_DimPersonPosition CHECK (person_position IN (
        'DRIVER',
        'FRONT',
        'REAR',
        'LAP',
        'OUTSIDE',
        'UNKNOWN'
    ))
);


-- =============================================================================
-- Dimension: Person Role
-- =============================================================================
CREATE TABLE dm.DimPersonRole (
    person_role_id      INT         NOT NULL,
    person_role         VARCHAR(20) NOT NULL,

    CONSTRAINT PK_DimPersonRole PRIMARY KEY CLUSTERED (person_role_id),

    CONSTRAINT CK_DimPersonRole CHECK (person_role IN (
        'NOTIFIED_PERSON',
        'WITNESS',
        'REGISTRANT',
        'IN_LINE_SKATER',
        'PASSENGER',
        'DRIVER',
        'POLICY_HOLDER',
        'OWNER',
        'PEDESTRIAN',
        'UNKNOWN'
    ))
);


-- =============================================================================
-- Dimension: Person Sex
-- =============================================================================
CREATE TABLE dm.DimPersonSex (
    person_sex_id   INT         NOT NULL,
    person_sex      VARCHAR(10) NOT NULL,

    CONSTRAINT PK_DimPersonSex PRIMARY KEY CLUSTERED (person_sex_id),

    CONSTRAINT CK_DimPersonSex CHECK (person_sex IN (
        'MALE',
        'FEMALE',
        'UNKNOWN'
    ))
);


-- =============================================================================
-- Dimension: Person Type
-- =============================================================================
CREATE TABLE dm.DimPersonType (
    person_type_id  INT         NOT NULL,
    person_type     VARCHAR(20) NOT NULL,

    CONSTRAINT PK_DimPersonType PRIMARY KEY CLUSTERED (person_type_id),

    CONSTRAINT CK_DimPersonType CHECK (person_type IN (
        'PEDESTRIAN',
        'OCCUPANT',
        'BICYCLIST',
        'OTHER_MOTORIZED',
        'UNKNOWN'
    ))
);


-- =============================================================================
-- Dimension: Contributing Factor
-- Hierarchy: factor → category → (typed sub-category)
--
--   Level 1 – contributing_factor            (leaf / most specific)
--   Level 2 – contributing_factor_hier_def_subcategory
--   Level 3 – contributing_factor_hier_def_category  (root / most general)
-- =============================================================================
CREATE TABLE dm.DimContributingFactor (
    contributing_factor_id                  INT         NOT NULL,
    contributing_factor                     VARCHAR(60) NOT NULL,
    contributing_factor_hier_def_category   VARCHAR(25) NOT NULL,
    contributing_factor_hier_def_subcategory VARCHAR(60) NOT NULL,

    CONSTRAINT PK_DimContributingFactor PRIMARY KEY CLUSTERED (contributing_factor_id),

    CONSTRAINT CK_DimContributingFactor_Factor CHECK (contributing_factor IN (
        'DRIVERLESS_RUNAWAY_VEHICLE',
        'LISTENING_USING_HEADPHONES',
        'EATING_OR_DRINKING',
        'UNSAFE_LANE_CHANGING',
        'CELL_PHONE_HAND_HELD',
        'CELL_PHONE_HANDS_FREE',
        'DRUGS_ILLEGAL',
        'TEXTING',
        'HEADLIGHTS_DEFECTIVE',
        'OTHER_LIGHTING_DEFECTS',
        'DRIVER_INEXPERIENCE',
        'AGGRESSIVE_DRIVING_ROAD_RAGE',
        'UNSAFE_SPEED',
        'LANE_MARKING_IMPROPER_INADEQUATE',
        'GLARE',
        'TRAFFIC_CONTROL_DEVICE_IMPROPER_NON_WORKING',
        'PASSING_TOO_CLOSELY',
        'ACCELERATOR_DEFECTIVE',
        'SHOULDERS_DEFECTIVE_IMPROPER',
        'OUTSIDE_CAR_DISTRACTION',
        'DRIVER_INATTENTION_DISTRACTION',
        'TINTED_WINDOWS',
        'USING_ON_BOARD_NAVIGATION_DEVICE',
        'REACTION_TO_OTHER_UNINVOLVED_VEHICLE',
        'OBSTRUCTION_DEBRIS',
        'PRESCRIPTION_MEDICATION',
        'TIRE_FAILURE_INADEQUATE',
        'FATIGUED_DROWSY',
        'PASSING_OR_LANE_USAGE_IMPROPER',
        'FOLLOWING_TOO_CLOSELY',
        'VIEW_OBSTRUCTED_LIMITED',
        'OVERSIZED_VEHICLE',
        'LOST_CONSCIOUSNESS',
        'BACKING_UNSAFELY',
        'OTHER_VEHICULAR',
        'ILLNESS',
        'WINDSHIELD_INADEQUATE',
        'FELL_ASLEEP',
        'TRAFFIC_CONTROL_DISREGARDED',
        'PAVEMENT_DEFECTIVE',
        'STEERING_FAILURE',
        'PASSENGER_DISTRACTION',
        'VEHICLE_VANDALISM',
        'FAILURE_TO_KEEP_RIGHT',
        'BRAKES_DEFECTIVE',
        'TURNING_IMPROPERLY',
        'FAILURE_TO_YIELD_RIGHT_OF_WAY',
        'REACTION_TO_UNINVOLVED_VEHICLE',
        'TOW_HITCH_DEFECTIVE',
        'ALCOHOL_INVOLVEMENT',
        'PHYSICAL_DISABILITY',
        'ANIMALS_ACTION',
        'OTHER_ELECTRONIC_DEVICE',
        'PEDESTRIAN_BICYCLIST_OTHER_PEDESTRIAN_ERROR_CONFUSION',
        'PAVEMENT_SLIPPERY',
        'UNKNOWN'
    )),

    CONSTRAINT CK_DimContributingFactor_Category CHECK (contributing_factor_hier_def_category IN (
        'HUMAN_BEHAVIOR',
        'HUMAN_CONDITION',
        'DISTRACTION',
        'SUBSTANCE_RELATED',
        'VEHICLE_DEFECT',
        'ROAD_INFRASTRUCTURE',
        'ENVIRONMENTAL',
        'EXTERNAL',
        'UNKNOWN'
    ))
);

CREATE INDEX IX_DimContributingFactor_Category
    ON dm.DimContributingFactor (contributing_factor_hier_def_category);


-- =============================================================================
-- Fact Table: Vehicle Incident Person
--
-- Grain: one row per person involved in a crash.
-- All measures are taken from the crash record the person belongs to,
-- so they express the total impact of the crash each person was part of.
--
-- Degenerate dimension: none (crash_id is in the base DB, not carried here).
-- =============================================================================
CREATE TABLE dm.Fact (
    -- Surrogate key
    fact_id                 INT      NOT NULL,

    -- Dimension foreign keys
    contributing_factor_id  INT      NOT NULL,
    person_age_id           INT      NOT NULL,
    person_position_id      INT      NOT NULL,
    person_role_id          INT      NOT NULL,
    person_sex_id           INT      NOT NULL,
    person_type_id          INT      NOT NULL,
    time_id                 INT      NOT NULL,

    -- Measures (additive)
    persons_injured         TINYINT  NOT NULL,
    persons_killed          TINYINT  NOT NULL,
    pedestrians_injured     TINYINT  NOT NULL,
    pedestrians_killed      TINYINT  NOT NULL,
    cyclist_injured         TINYINT  NOT NULL,
    cyclist_killed          TINYINT  NOT NULL,
    motorist_injured        TINYINT  NOT NULL,
    motorist_killed         TINYINT  NOT NULL,

    CONSTRAINT PK_Fact PRIMARY KEY CLUSTERED (fact_id),

    CONSTRAINT FK_Fact_Time
        FOREIGN KEY (time_id)
        REFERENCES dm.DimTime (time_id),

    CONSTRAINT FK_Fact_PersonAge
        FOREIGN KEY (person_age_id)
        REFERENCES dm.DimPersonAge (person_age_id),

    CONSTRAINT FK_Fact_PersonPosition
        FOREIGN KEY (person_position_id)
        REFERENCES dm.DimPersonPosition (person_position_id),

    CONSTRAINT FK_Fact_PersonRole
        FOREIGN KEY (person_role_id)
        REFERENCES dm.DimPersonRole (person_role_id),

    CONSTRAINT FK_Fact_PersonSex
        FOREIGN KEY (person_sex_id)
        REFERENCES dm.DimPersonSex (person_sex_id),

    CONSTRAINT FK_Fact_PersonType
        FOREIGN KEY (person_type_id)
        REFERENCES dm.DimPersonType (person_type_id),

    CONSTRAINT FK_Fact_ContributingFactor
        FOREIGN KEY (contributing_factor_id)
        REFERENCES dm.DimContributingFactor (contributing_factor_id),

    CONSTRAINT CK_Fact_NonNegative CHECK (
        persons_injured     >= 0 AND
        persons_killed      >= 0 AND
        pedestrians_injured >= 0 AND
        pedestrians_killed  >= 0 AND
        cyclist_injured     >= 0 AND
        cyclist_killed      >= 0 AND
        motorist_injured    >= 0 AND
        motorist_killed     >= 0
    )
);

-- Covering indexes to accelerate common analytical queries.
CREATE INDEX IX_Fact_Time
    ON dm.Fact (time_id)
    INCLUDE (persons_injured, persons_killed);

CREATE INDEX IX_Fact_PersonSex
    ON dm.Fact (person_sex_id)
    INCLUDE (persons_injured, persons_killed);

CREATE INDEX IX_Fact_PersonAge
    ON dm.Fact (person_age_id)
    INCLUDE (persons_injured, persons_killed);

CREATE INDEX IX_Fact_PersonType
    ON dm.Fact (person_type_id)
    INCLUDE (persons_injured, persons_killed);

CREATE INDEX IX_Fact_ContributingFactor
    ON dm.Fact (contributing_factor_id)
    INCLUDE (persons_injured, persons_killed);
