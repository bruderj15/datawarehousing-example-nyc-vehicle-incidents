use datawarehousing_example_nyc_vehicle_incidents::{
    base_database::time::Time,
    raw::{moon::RawMoonRecord, weather::RawWeatherRecord},
};

fn main() {
    let raw_moon = RawMoonRecord::load_from_csv("data/moon.csv");
    let raw_weather = RawWeatherRecord::load_from_csv("data/weather.csv");

    for x in Time::from(raw_weather, raw_moon) {
        println!("{x:?}");
    }
}
