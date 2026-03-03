use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime, macros::format_description};

use crate::base_database;

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

impl Time {
    pub fn gen_times(bdb_times: Vec<base_database::time::Time>) -> Vec<Time> {
        let month_format = format_description!("[month repr:long]");
        bdb_times
            .into_iter()
            .enumerate()
            .map(|(i, bdd_time)| Time {
                time_id: i as u32 + 1, // start from 1 to avoid confusion with conventional uninitialized value of 0
                timestamp: bdd_time.timestamp,
                hier_def_day: bdd_time.timestamp.date(),
                hier_def_month: bdd_time
                    .timestamp
                    .format(&month_format)
                    .expect("shouldn't fail formatting Month from Date"),
                hier_def_year: bdd_time.timestamp.year() as u16,
                hier_moon_phase: bdd_time
                    .moon_phase
                    .map(MoonPhase::from)
                    .unwrap_or(MoonPhase::Unknown),
                weather: bdd_time
                    .weather
                    .map(Weather::from)
                    .unwrap_or(Weather::Unknown),
            })
            .collect()
    }
}

impl From<base_database::time::MoonPhase> for MoonPhase {
    fn from(moon_phase: base_database::time::MoonPhase) -> Self {
        match moon_phase {
            base_database::time::MoonPhase::New => MoonPhase::New,
            base_database::time::MoonPhase::WaxingCrescent => MoonPhase::WaxingCrescent,
            base_database::time::MoonPhase::FirstQuarter => MoonPhase::FirstQuarter,
            base_database::time::MoonPhase::WaxingGibbous => MoonPhase::WaxingGibbous,
            base_database::time::MoonPhase::Full => MoonPhase::Full,
            base_database::time::MoonPhase::WaningGibbous => MoonPhase::WaningGibbous,
            base_database::time::MoonPhase::LastQuarter => MoonPhase::LastQuarter,
            base_database::time::MoonPhase::WaningCrescent => MoonPhase::WaningCrescent,
        }
    }
}

impl From<base_database::time::Weather> for Weather {
    fn from(weather: base_database::time::Weather) -> Self {
        match weather {
            base_database::time::Weather::Clear => Weather::Clear,
            base_database::time::Weather::Cloudy => Weather::Cloudy,
            base_database::time::Weather::RainyLight => Weather::RainyLight,
            base_database::time::Weather::RainyHeavy => Weather::RainyHeavy,
            base_database::time::Weather::Stormy => Weather::Stormy,
            base_database::time::Weather::Windy => Weather::Windy,
            base_database::time::Weather::Miscallaneous => Weather::Miscallaneous,
        }
    }
}
