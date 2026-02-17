use datawarehousing_example_nyc_vehicle_incidents::{
    base_database::person::Person, raw::persons::RawPersonRecord,
};
use std::collections::HashSet;

fn main() {
    let xs = RawPersonRecord::load_from_csv("data/persons.csv")
        .into_iter()
        .map(Person::from);

    for x in xs {
        println!("{x:?}");
    }
}
