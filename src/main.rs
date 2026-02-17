use datawarehousing_example_nyc_vehicle_incidents::{
    base_database::{crash::Crash, person::Person},
    raw::{crashes::RawCrashRecord, persons::RawPersonRecord},
};
use std::collections::HashSet;

fn main() {
    let xs = RawCrashRecord::load_from_csv("data/crashes.csv")
        .into_iter()
        .map(Crash::from);

    for x in xs {
        println!("{x:?}");
    }
}
