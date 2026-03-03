use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

#[derive(Debug, Clone)]
pub struct Time {
    pub time_id: u32,
    pub timestamp: OffsetDateTime,

    // default hierarchy
    pub hier_def_day: Date,
    pub hier_def_month: String,
    pub hier_def_year: u16,

    // moon hierarchy
    pub hier_moon_phase: MoonPhase,

    // denormalized weather
    pub weather: Weather,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoonPhase {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Weather {
    Clear,
    Cloudy,
    RainyLight,
    RainyHeavy,
    Stormy,
    Windy,
    Miscallaneous,
    Unknown,
}
