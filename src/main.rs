use datawarehousing_example_nyc_vehicle_incidents::{
    base_database::{crash::Crash, person::Person, time::Time as BdbTime},
    data_mart::{
        contributing_factor::ContributingFactorDim, fact::Fact, person_age::PersonAge,
        person_position::PersonPosition, person_role::PersonPositionRole, person_sex::PersonSex,
        person_type::PersonType, time::Time as DmTime,
    },
    raw::{
        crashes::RawCrashRecord, moon::RawMoonRecord, persons::RawPersonRecord,
        weather::RawWeatherRecord,
    },
};
use std::fs;

fn main() {
    // -----------------------------------------------------------------------
    // Stage 1: Load raw data
    // -----------------------------------------------------------------------
    println!("[1/5] Loading raw data...");

    let raw_moon = RawMoonRecord::load_from_csv("data/moon.csv");
    println!("      moon records:    {}", raw_moon.len());

    let raw_weather = RawWeatherRecord::load_from_csv("data/weather.csv");
    println!("      weather records: {}", raw_weather.len());

    let raw_crashes = RawCrashRecord::load_from_csv("data/crashes.csv");
    println!("      crash records:   {}", raw_crashes.len());

    let raw_persons = RawPersonRecord::load_from_csv("data/persons.csv");
    println!("      person records:  {}", raw_persons.len());

    // -----------------------------------------------------------------------
    // Stage 2: Build base database
    // -----------------------------------------------------------------------
    println!("[2/5] Building base database...");

    let bdb_times: Vec<BdbTime> = BdbTime::from(raw_weather, raw_moon);
    println!("      time rows:   {}", bdb_times.len());

    // Build a lookup from truncated-hour timestamp → time_id for crash linking.
    let time_lookup: std::collections::HashMap<time::PrimitiveDateTime, u32> =
        bdb_times.iter().map(|t| (t.timestamp, t.time_id)).collect();

    let bdb_crashes: Vec<Crash> = raw_crashes
        .into_iter()
        .map(|raw| {
            let crash = Crash::from(raw);
            // Truncate the crash timestamp to the hour to find the matching time row.
            let hour_ts = crash
                .crash_timestamp
                .replace_minute(0)
                .and_then(|t| t.replace_second(0))
                .and_then(|t| t.replace_nanosecond(0))
                .ok();
            let time_id = hour_ts.and_then(|ts| time_lookup.get(&ts).copied());
            match time_id {
                Some(id) => crash.with_time_id(id),
                None => crash,
            }
        })
        .collect();
    println!("      crash rows:  {}", bdb_crashes.len());

    let bdb_persons: Vec<Person> = raw_persons.into_iter().map(Person::from).collect();
    println!("      person rows: {}", bdb_persons.len());

    // -----------------------------------------------------------------------
    // Stage 3: Build data mart dimension tables
    // -----------------------------------------------------------------------
    println!("[3/5] Building data mart dimensions...");

    let dm_times: Vec<DmTime> = DmTime::gen_times(bdb_times);
    println!("      dim_time rows:               {}", dm_times.len());

    let dim_ages: Vec<PersonAge> = PersonAge::gen_ages();
    println!("      dim_person_age rows:          {}", dim_ages.len());

    let dim_positions: Vec<PersonPosition> = PersonPosition::gen_positions();
    println!(
        "      dim_person_position rows:     {}",
        dim_positions.len()
    );

    let dim_roles: Vec<PersonPositionRole> = PersonPositionRole::gen_positions_roles();
    println!("      dim_person_role rows:         {}", dim_roles.len());

    let dim_sexes: Vec<PersonSex> = PersonSex::gen_sexes();
    println!("      dim_person_sex rows:          {}", dim_sexes.len());

    let dim_types: Vec<PersonType> = PersonType::gen_types();
    println!("      dim_person_type rows:         {}", dim_types.len());

    let dim_factors: Vec<ContributingFactorDim> = ContributingFactorDim::gen_factors();
    println!("      dim_contributing_factor rows: {}", dim_factors.len());

    // -----------------------------------------------------------------------
    // Stage 4: Build fact table
    // -----------------------------------------------------------------------
    println!("[4/5] Building fact table...");

    let facts: Vec<Fact> = Fact::gen_facts(
        bdb_persons,
        bdb_crashes,
        &dm_times,
        &dim_ages,
        &dim_positions,
        &dim_roles,
        &dim_sexes,
        &dim_types,
        &dim_factors,
    );
    println!("      fact rows: {}", facts.len());

    // -----------------------------------------------------------------------
    // Stage 5: Write output files
    // -----------------------------------------------------------------------
    fs::create_dir_all("data/output").expect("failed to create data/output directory");

    // -- JSON ----------------------------------------------------------------
    println!("[5/6] Writing JSON files to data/output/...");

    write_json("data/output/dim_time.json", &dm_times);
    write_json("data/output/dim_person_age.json", &dim_ages);
    write_json("data/output/dim_person_position.json", &dim_positions);
    write_json("data/output/dim_person_role.json", &dim_roles);
    write_json("data/output/dim_person_sex.json", &dim_sexes);
    write_json("data/output/dim_person_type.json", &dim_types);
    write_json("data/output/dim_contributing_factor.json", &dim_factors);
    write_json("data/output/fact.json", &facts);

    println!("Done.");
}

fn write_json<T: serde::Serialize>(path: &str, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("failed to serialize to JSON");
    fs::write(path, json).unwrap_or_else(|e| panic!("failed to write {path}: {e}"));
    println!("      wrote {path}");
}
