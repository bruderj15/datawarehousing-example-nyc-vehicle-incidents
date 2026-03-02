CREATE TABLE dbo.[Time] (
    time_id            INT            NOT NULL,
    [timestamp]        DATETIMEOFFSET(0) NOT NULL, -- RFC3339 compatible
    moon_phase         VARCHAR(20)    NULL,
    weather            VARCHAR(20)    NULL,

    CONSTRAINT PK_Time PRIMARY KEY CLUSTERED (time_id),

    CONSTRAINT CK_Time_MoonPhase CHECK (moon_phase IS NULL OR moon_phase IN (
        'NEW',
        'WAXING_CRESCENT',
        'FIRST_QUARTER',
        'WAXING_GIBBOUS',
        'FULL',
        'WANING_GIBBOUS',
        'LAST_QUARTER',
        'WANING_CRESCENT'
    )),

    CONSTRAINT CK_Time_Weather CHECK (weather IS NULL OR weather IN (
        'CLEAR',
        'CLOUDY',
        'RAINY_LIGHT',
        'RAINY_HEAVY',
        'STORMY',
        'WINDY',
        'MISCALLANEOUS'
    ))
);

CREATE INDEX IX_Time_Timestamp ON dbo.[Time] ([timestamp]);


------------------


CREATE TABLE dbo.Crash (
    crash_id                           INT               NOT NULL,
    crash_timestamp                    DATETIMEOFFSET(0) NOT NULL,
    crash_persons_injured              SMALLINT          NOT NULL,
    crash_persons_killed               SMALLINT          NOT NULL,
    crash_pedestrians_injured          SMALLINT          NOT NULL,
    crash_pedestrians_killed           SMALLINT          NOT NULL,
    crash_cyclist_injured              SMALLINT          NOT NULL,
    crash_cyclist_killed               SMALLINT          NOT NULL,
    crash_motorist_injured             SMALLINT          NOT NULL,
    crash_motorist_killed              SMALLINT          NOT NULL,
    crash_factor                       VARCHAR(60)       NULL,
    time_id                            INT               NULL,

    CONSTRAINT PK_Crash PRIMARY KEY CLUSTERED (crash_id),

    CONSTRAINT FK_Crash_Time
        FOREIGN KEY (time_id)
        REFERENCES dbo.[Time](time_id),

    CONSTRAINT CK_Crash_NonNegative CHECK (
        crash_persons_injured >= 0 AND
        crash_persons_killed >= 0 AND
        crash_pedestrians_injured >= 0 AND
        crash_pedestrians_killed >= 0 AND
        crash_cyclist_injured >= 0 AND
        crash_cyclist_killed >= 0 AND
        crash_motorist_injured >= 0 AND
        crash_motorist_killed >= 0
    ),

    CONSTRAINT CK_Crash_Factor CHECK (crash_factor IS NULL OR crash_factor IN (
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
        'PAVEMENT_SLIPPERY'
    ))
);

CREATE INDEX IX_Crash_TimeId ON dbo.Crash (time_id);
CREATE INDEX IX_Crash_Timestamp ON dbo.Crash (crash_timestamp);


------------------


CREATE TABLE dbo.Person (
    person_id                   INT           NOT NULL,
    person_type                 VARCHAR(20)   NULL,
    person_age                  TINYINT       NULL,
    person_sex                  VARCHAR(10)   NULL,
    person_position_in_vehicle  VARCHAR(20)   NULL,
    person_role                 VARCHAR(30)   NULL,
    crash_id                    INT           NOT NULL,

    CONSTRAINT PK_Person PRIMARY KEY CLUSTERED (person_id),

    CONSTRAINT FK_Person_Crash
        FOREIGN KEY (crash_id)
        REFERENCES dbo.Crash(crash_id)
        ON DELETE CASCADE,

    CONSTRAINT CK_Person_Age CHECK (
        person_age IS NULL OR person_age BETWEEN 0 AND 120
    ),

    CONSTRAINT CK_Person_Sex CHECK (person_sex IS NULL OR person_sex IN (
        'MALE',
        'FEMALE'
    )),

    CONSTRAINT CK_Person_Type CHECK (person_type IS NULL OR person_type IN (
        'PEDESTRIAN',
        'OCCUPANT',
        'BICYCLIST',
        'OTHER_MOTORIZED'
    )),

    CONSTRAINT CK_Person_Position CHECK (person_position_in_vehicle IS NULL OR person_position_in_vehicle IN (
        'DRIVER',
        'FRONT',
        'REAR',
        'LAP',
        'OUTSIDE'
    )),

    CONSTRAINT CK_Person_Role CHECK (person_role IS NULL OR person_role IN (
        'NOTIFIED_PERSON',
        'WITNESS',
        'REGISTRANT',
        'IN_LINE_SKATER',
        'PASSENGER',
        'DRIVER',
        'POLICY_HOLDER',
        'OWNER',
        'PEDESTRIAN'
    ))
);

CREATE INDEX IX_Person_CrashId ON dbo.Person (crash_id);
