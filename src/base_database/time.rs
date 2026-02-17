use crate::raw::{moon::RawMoonRecord, weather::RawWeatherRecord};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use time::{Date, Duration, OffsetDateTime, macros::datetime};

#[derive(Debug, Clone)]
pub struct Time {
    pub time_id: u32,
    pub timestamp: OffsetDateTime,
    pub moon_phase: Option<MoonPhase>,
    pub temperature: Option<f32>,
    pub precipitation: Option<f32>,
    pub rain: Option<f32>,
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

impl Time {
    pub fn from(raw_weather: Vec<RawWeatherRecord>, raw_moon: Vec<RawMoonRecord>) -> Vec<Time> {
        let moon = extract_moon_phases(raw_moon);
        let weather = raw_weather
            .into_iter()
            .map(|raw_weather| (raw_weather.time, raw_weather))
            .collect::<HashMap<_, _>>();

        let start = datetime!(2000 - 01 - 01 0:00 UTC);
        let end = datetime!(2022 - 12 - 31 0:00 UTC);
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
                temperature: weather.map(|weather| weather.temperature),
                precipitation: weather.map(|weather| weather.precipitation),
                rain: weather.map(|weather| weather.rain),
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
