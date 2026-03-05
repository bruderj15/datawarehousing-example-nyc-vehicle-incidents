use crate::raw::{moon::RawMoonRecord, weather::RawWeatherRecord};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use time::{Date, Duration, PrimitiveDateTime, macros::date};

#[derive(Debug, Clone)]
pub struct Time {
    pub time_id: u32,
    pub timestamp: PrimitiveDateTime,
    pub moon_phase: Option<MoonPhase>,
    pub weather: Option<Weather>,
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
}

/// | Condition   | Logic                      |
/// | ----------- | -------------------------- |
/// | Clear       | Cloud < 20% AND Precip = 0 |
/// | Cloudy      | Cloud ≥ 60% AND Precip = 0 |
/// | Rainy Light | Rain ≤ 2.5                 |
/// | Rainy Heavy | Rain > 2.5                 |
/// | Stormy      | Rain > 7.5 AND Wind > 40   |
/// | Windy       | Wind > 40 AND Rain = 0     |
impl From<&RawWeatherRecord> for Weather {
    fn from(raw_weather: &RawWeatherRecord) -> Self {
        if raw_weather.precipitation == 0.0 && raw_weather.rain == 0.0 {
            if raw_weather.cloudcover < 20.0 {
                Weather::Clear
            } else if raw_weather.cloudcover >= 60.0 {
                Weather::Cloudy
            } else {
                Weather::Miscallaneous
            }
        } else if raw_weather.rain <= 2.5 {
            Weather::RainyLight
        } else if raw_weather.rain > 2.5 {
            if raw_weather.windspeed > 40.0 {
                Weather::Stormy
            } else {
                Weather::RainyHeavy
            }
        } else if raw_weather.windspeed > 40.0 && raw_weather.rain == 0.0 {
            Weather::Windy
        } else {
            Weather::Miscallaneous
        }
    }
}

impl Time {
    pub fn from(raw_weather: Vec<RawWeatherRecord>, raw_moon: Vec<RawMoonRecord>) -> Vec<Time> {
        let moon = extract_moon_phases(raw_moon);
        let weather = raw_weather
            .into_iter()
            .map(|raw_weather| {
                (
                    PrimitiveDateTime::new(raw_weather.time.date(), raw_weather.time.time()),
                    raw_weather,
                )
            })
            .collect::<HashMap<_, _>>();

        let start = PrimitiveDateTime::new(date!(2016 - 01 - 01), time::macros::time!(0:00));
        let end = PrimitiveDateTime::new(date!(2022 - 12 - 31), time::macros::time!(23:59));
        std::iter::successors(Some(start), move |&dt| {
            let next = dt + Duration::hours(1);
            if next <= end { Some(next) } else { None }
        })
        .enumerate()
        .map(|(i, timestamp)| {
            let moon = moon.get(&timestamp.date());
            let weather = weather.get(&timestamp);
            Time {
                time_id: i as u32,
                timestamp,
                moon_phase: moon.copied(),
                weather: weather.map(Weather::from),
            }
        })
        .collect()
    }
}

fn extract_moon_phases(raw_moon: Vec<RawMoonRecord>) -> HashMap<Date, MoonPhase> {
    let moon_change = raw_moon
        .into_iter()
        .flat_map(|moon_record| {
            let mut moon_phases = BTreeMap::new();
            if let Some(date) = moon_record.new_moon {
                moon_phases.insert(date, MoonPhase::New);
            }
            if let Some(date) = moon_record.first_quarter_moon {
                moon_phases.insert(date, MoonPhase::FirstQuarter);
            }
            if let Some(date) = moon_record.full_moon {
                moon_phases.insert(date, MoonPhase::Full);
            }
            if let Some(date) = moon_record.third_quarter_moon {
                moon_phases.insert(date, MoonPhase::LastQuarter);
            }
            moon_phases
        })
        .collect::<BTreeMap<_, _>>();

    let mut moon = HashMap::new();
    for ((date1, phase1), (date2, _)) in moon_change.into_iter().tuple_windows() {
        std::iter::successors(Some(date1), move |&dt| {
            let next = dt + Duration::days(1);
            if next < date2 { Some(next) } else { None }
        })
        .for_each(|date| {
            moon.insert(date, phase1);
        });
    }

    moon
}
