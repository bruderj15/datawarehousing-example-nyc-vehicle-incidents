use datawarehousing_example_nyc_vehicle_incidents::{
    base_database::{crash::Crash, person::Person},
    raw::{crashes::RawCrashRecord, moon::RawMoonRecord, persons::RawPersonRecord},
};
use std::collections::HashSet;

fn main() {
    let xs = RawMoonRecord::load_from_csv("data/moon.csv").into_iter();

    for x in xs {
        println!("{x:?}");
    }
}
