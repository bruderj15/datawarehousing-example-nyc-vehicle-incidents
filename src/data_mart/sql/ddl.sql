-- =============================================================================
-- Data Mart DDL  –  NYC Vehicle Incidents Star Schema
-- =============================================================================
-- Naming conventions:
--   Dimension tables : project_julian_bruder_kenana_saeed.<DimName>
--   Fact table       : project_julian_bruder_kenana_saeed.Fact
-- All surrogate keys are INT; 0 is reserved as the "Unknown" sentinel.
-- =============================================================================
-- PRIMARY ANALYTICS QUESTION
--   Does a female person's menstrual-cycle phase (proxied by moon phase +
--   fertile age group) correlate with incident severity, independent of
--   weather and contributing-factor category?
--
-- INDEXING STRATEGY OVERVIEW
--   Fact table  → Clustered Columnstore Index (CCI).
--                 The fact table is wide, append-only, and queried exclusively
--                 via large aggregating scans — the classic CCI use-case.
--
--   Dimensions  → Clustered B-tree PKs (small tables, point-lookup on SK is
--                 the only access pattern the optimizer needs from them).
--                 Selective non-clustered indexes are added only where the
--                 analytical queries filter or group on non-SK columns with
--                 meaningful cardinality.
--
--   Materialized view → One pre-aggregated indexed view at the grain of
--                 (moon_phase × weather × factor_category × person_sex ×
--                  age_group) collapses the multi-billion-row star join into
--                 ~1 000 rows the reporting layer reads in probably microseconds.
-- =============================================================================


-- =============================================================================
-- Dimension: Time
-- Parallel hierarchies:
--   Default  : timestamp → day → month → year
--   Moon     : timestamp → moon_phase
-- Denormalized weather attribute stored directly on the dimension row.
-- =============================================================================
CREATE TABLE project_julian_bruder_kenana_saeed.DimTime (
    time_id                 INT               NOT NULL,
    [timestamp]             DATETIMEOFFSET(0) NOT NULL,

    -- Default hierarchy
    hier_def_day            DATE              NOT NULL,
    hier_def_month          VARCHAR(12)       NOT NULL,   -- e.g. 'January'
    hier_def_year           SMALLINT          NOT NULL,

    -- Moon-phase hierarchy
    hier_moon_phase         VARCHAR(20)       NOT NULL,

    -- Denormalized weather attribute
    weather                 VARCHAR(20)       NOT NULL,

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


-- Day range-scan for calendar-based slicing (e.g. "all of March").
-- This is the finest temporal grain queries actually filter on.
CREATE INDEX IX_DimTime_Day
    ON project_julian_bruder_kenana_saeed.DimTime (hier_def_day);

-- Composite: moon_phase leading, weather included.
--   The primary research question always slices on moon_phase AND controls for
--   weather simultaneously.  A single composite index satisfies both filter
--   predicates and covers weather without a key lookup, making a separate
--   IX_DimTime_Weather redundant.
--   moon_phase leads because it has higher cardinality (9 values) than
--   hier_def_year (handful of years), giving better selectivity up front.
-- Thought: Wonder if this index is relevant as we have CCI Fact-Table
CREATE INDEX IX_DimTime_MoonPhase_Weather
    ON project_julian_bruder_kenana_saeed.DimTime (hier_moon_phase, weather)
    INCLUDE (hier_def_year, hier_def_month);
-- Note: hier_def_year / hier_def_month are INCLUDEd (not key columns) because
--   queries group by them after filtering on phase+weather; they do not need
--   to drive range ordering.


-- =============================================================================
-- Dimension: Person Age
-- Hierarchy: age → age_group (Fertile / Infertile / Unknown)
-- Row 0 is the "age unknown" sentinel.
-- =============================================================================
CREATE TABLE project_julian_bruder_kenana_saeed.DimPersonAge (
    person_age_id               INT         NOT NULL,
    person_age                  TINYINT     NOT NULL,   -- 0 means "not known"
    person_age_known            BIT         NOT NULL,   -- person_age != 0
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
CREATE TABLE project_julian_bruder_kenana_saeed.DimPersonPosition (
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
CREATE TABLE project_julian_bruder_kenana_saeed.DimPersonRole (
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
CREATE TABLE project_julian_bruder_kenana_saeed.DimPersonSex (
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
CREATE TABLE project_julian_bruder_kenana_saeed.DimPersonType (
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
-- Hierarchy: factor → sub-category → category
--
--   Level 1 – contributing_factor            (leaf / most specific)
--   Level 2 – contributing_factor_hier_def_subcategory
--   Level 3 – contributing_factor_hier_def_category  (root / most general)
-- =============================================================================
CREATE TABLE project_julian_bruder_kenana_saeed.DimContributingFactor (
    contributing_factor_id                   INT         NOT NULL,
    contributing_factor                      VARCHAR(60) NOT NULL,
    contributing_factor_hier_def_category    VARCHAR(25) NOT NULL,
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


-- =============================================================================
-- Fact Table: Vehicle Incident Person
--
-- Grain: one row per person involved in a crash.
-- All measures are taken from the crash record the person belongs to,
-- so they express the total impact of the crash each person was part of.
--
-- WHY CLUSTERED COLUMNSTORE (CCI) INSTEAD OF A CLUSTERED B-TREE:
--   1. Access pattern: every analytical query performs a full or near-full
--      scan of the fact table followed by aggregation (SUM/COUNT).  A B-tree
--      clustered on a meaningless surrogate key (fact_id) gives zero range-scan
--      benefit because fact_id has no relationship to any filter predicate.
--   2. Compression: the CCI applies per-column compression (RLE + bit-packing).
--      All FK columns are low-cardinality integers; compression ratios of
--      10–20x are typical, drastically reducing I/O.
--   3. Batch-mode execution: the query engine processes 900-row batches instead
--      of row-by-row, yielding 5–100x CPU improvements for aggregate queries.
--   4. Segment elimination: the CCI stores min/max metadata per row-group
--      (~1 M rows).  Predicates on time_id, person_sex_id, etc. allow the
--      engine to skip entire compressed segments without decompression.
--   5. Write profile: a data-warehouse fact table is bulk-loaded periodically
--      (ETL), not updated row-by-row.  Delta-store overhead from the CCI is
--      negligible because loads are done in large batches.
--
--   The PRIMARY KEY is kept as NONCLUSTERED to preserve FK integrity and to
--   support ETL upsert checks (existence lookups by surrogate key).  It is
--   never used by analytical queries.
-- =============================================================================
CREATE TABLE project_julian_bruder_kenana_saeed.Fact (
    -- Surrogate key
    fact_id                 INT     NOT NULL,

    -- Dimension foreign keys
    contributing_factor_id  INT     NOT NULL,
    person_age_id           INT     NOT NULL,
    person_position_id      INT     NOT NULL,
    person_role_id          INT     NOT NULL,
    person_sex_id           INT     NOT NULL,
    person_type_id          INT     NOT NULL,
    time_id                 INT     NOT NULL,

    -- Measures (additive)
    persons_injured         TINYINT NOT NULL,
    persons_killed          TINYINT NOT NULL,
    pedestrians_injured     TINYINT NOT NULL,
    pedestrians_killed      TINYINT NOT NULL,
    cyclist_injured         TINYINT NOT NULL,
    cyclist_killed          TINYINT NOT NULL,
    motorist_injured        TINYINT NOT NULL,
    motorist_killed         TINYINT NOT NULL,

    -- PK is NONCLUSTERED: the CCI (below) is the physical storage order.
    CONSTRAINT PK_Fact PRIMARY KEY NONCLUSTERED (fact_id),

    CONSTRAINT FK_Fact_Time
        FOREIGN KEY (time_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimTime (time_id),

    CONSTRAINT FK_Fact_PersonAge
        FOREIGN KEY (person_age_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimPersonAge (person_age_id),

    CONSTRAINT FK_Fact_PersonPosition
        FOREIGN KEY (person_position_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimPersonPosition (person_position_id),

    CONSTRAINT FK_Fact_PersonRole
        FOREIGN KEY (person_role_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimPersonRole (person_role_id),

    CONSTRAINT FK_Fact_PersonSex
        FOREIGN KEY (person_sex_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimPersonSex (person_sex_id),

    CONSTRAINT FK_Fact_PersonType
        FOREIGN KEY (person_type_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimPersonType (person_type_id),

    CONSTRAINT FK_Fact_ContributingFactor
        FOREIGN KEY (contributing_factor_id)
        REFERENCES project_julian_bruder_kenana_saeed.DimContributingFactor (contributing_factor_id),

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

-- Physical storage: Clustered Columnstore Index.

-- All columns are implicitly included; no explicit column list is needed or
-- desired — partial CCIs would prevent batch-mode on excluded columns.
CREATE CLUSTERED COLUMNSTORE INDEX CCI_Fact
    ON project_julian_bruder_kenana_saeed.Fact;


-- =============================================================================
-- Materialized View: Severity by Moon Phase, Weather, Factor, Sex and Age Group
--
-- WHY THIS VIEW:
--   The primary research question requires joining Fact to four dimensions
--   (DimTime, DimPersonSex, DimPersonAge, DimContributingFactor) and then
--   grouping by (moon_phase, weather, factor_category, person_sex, age_group).
--   The combinatorial space of those five grouping columns has at most
--   9 × 8 × 9 × 3 × 3 = 5832 distinct cells — tiny.
--
--   Pre-aggregating all additive severity measures into this view means:
--     • Reports read ~6 000 rows instead of millions of fact rows.
--     • Confounding-variable analysis (e.g. "hold weather constant, vary phase")
--       is an in-memory operation on the view result set.
--     • The indexed view is maintained incrementally by SQL Server; ETL
--       refreshes propagate automatically with no manual refresh step.
--
-- SCOPE NOTE:
--   incident_count counts fact rows (people) in each cell, not unique crashes.
--   Severity sums are additive across the grain, consistent with the fact table
--   definition.  Consumers who need crash-level counts must apply additional
--   logic (not in scope of this mart layer).
-- =============================================================================
CREATE VIEW project_julian_bruder_kenana_saeed.MV_SeverityByMoonWeatherFactorSexAge
WITH SCHEMABINDING
AS
    SELECT
        -- Grouping dimensions (the five axes of the research question)
        dt.hier_moon_phase                              AS moon_phase,
        dt.weather                                      AS weather,
        dcf.contributing_factor_hier_def_category       AS factor_category,
        dps.person_sex                                  AS person_sex,
        dpa.person_age_hier_def_group                   AS age_group,

        -- Severity measures (all additive — SUM is safe across the grain)
        SUM(CAST(f.persons_injured     AS INT))         AS total_persons_injured,
        SUM(CAST(f.persons_killed      AS INT))         AS total_persons_killed,
        SUM(CAST(f.pedestrians_injured AS INT))         AS total_pedestrians_injured,
        SUM(CAST(f.pedestrians_killed  AS INT))         AS total_pedestrians_killed,
        SUM(CAST(f.cyclist_injured     AS INT))         AS total_cyclist_injured,
        SUM(CAST(f.cyclist_killed      AS INT))         AS total_cyclist_killed,
        SUM(CAST(f.motorist_injured    AS INT))         AS total_motorist_injured,
        SUM(CAST(f.motorist_killed     AS INT))         AS total_motorist_killed,

        -- Row count required by SQL Server for indexed view maintenance
        COUNT_BIG(*)                                    AS incident_count

    FROM project_julian_bruder_kenana_saeed.Fact                    AS f
    JOIN project_julian_bruder_kenana_saeed.DimTime                 AS dt  ON dt.time_id                = f.time_id
    JOIN project_julian_bruder_kenana_saeed.DimPersonSex            AS dps ON dps.person_sex_id         = f.person_sex_id
    JOIN project_julian_bruder_kenana_saeed.DimPersonAge            AS dpa ON dpa.person_age_id         = f.person_age_id
    JOIN project_julian_bruder_kenana_saeed.DimContributingFactor   AS dcf ON dcf.contributing_factor_id = f.contributing_factor_id

    GROUP BY
        dt.hier_moon_phase,
        dt.weather,
        dcf.contributing_factor_hier_def_category,
        dps.person_sex,
        dpa.person_age_hier_def_group;
GO

-- Unique clustered index — required by SQL Server to persist ("materialize")
-- the view to disk.  The five grouping columns together form the natural key
-- of the aggregated result set; uniqueness is guaranteed by the GROUP BY.
-- Clustering on (moon_phase, weather) first matches the most common query
-- pattern: "for each phase, controlling for weather, show severity".
-- person_sex and age_group come next so that the FEMALE/FERTILE cohort can be
-- isolated by a range seek without scanning the full view.
CREATE UNIQUE CLUSTERED INDEX UCI_MV_SeverityByMoonWeatherFactorSexAge
    ON project_julian_bruder_kenana_saeed.MV_SeverityByMoonWeatherFactorSexAge
        (moon_phase, weather, person_sex, age_group, factor_category);
