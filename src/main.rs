use datawarehousing_example_nyc_vehicle_incidents::{
    base_database::{crash::Crash, person::Person, time::Time as BdbTime},
    data_mart::{
        contributing_factor::ContributingFactorDim, fact::Fact, person_age::PersonAge,
        person_position::PersonPosition, person_role::PersonPositionRole, person_sex::PersonSex,
        person_type::PersonType, time::Time as DmTime,
    },
    ingestion::{DataMart, DataMartTable, DbCredentials},
    raw::{
        crashes::RawCrashRecord, moon::RawMoonRecord, persons::RawPersonRecord,
        weather::RawWeatherRecord,
    },
};
use std::fs;

#[tokio::main]
async fn main() {
    // -----------------------------------------------------------------------
    // Stage 1: Load raw data
    // -----------------------------------------------------------------------
    println!("[1/7] Loading raw data...");

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
    println!("[2/7] Building base database...");

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
    println!("[3/7] Building data mart dimensions...");

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
    println!("[4/7] Building fact table...");

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

    println!("[5/7] Writing JSON files to data/output/...");

    write_json("data/output/dim_time.json", &dm_times);
    write_json("data/output/dim_person_age.json", &dim_ages);
    write_json("data/output/dim_person_position.json", &dim_positions);
    write_json("data/output/dim_person_role.json", &dim_roles);
    write_json("data/output/dim_person_sex.json", &dim_sexes);
    write_json("data/output/dim_person_type.json", &dim_types);
    write_json("data/output/dim_contributing_factor.json", &dim_factors);
    write_json("data/output/fact.json", &facts);

    // -----------------------------------------------------------------------
    // Stage 6: Set up database schema (DDL)
    // -----------------------------------------------------------------------
    println!("[6/7] Setting up data mart schema in SQL Server...");

    let creds = db_credentials_from_env();

    if let Err(e) =
        datawarehousing_example_nyc_vehicle_incidents::ingestion::setup_data_mart(&creds).await
    {
        eprintln!("      ERROR during DDL setup: {e:#}");
        eprintln!("      Skipping ingestion. Fix the error and re-run.");
        return;
    }

    // -----------------------------------------------------------------------
    // Stage 7: Ingest data into the database
    // -----------------------------------------------------------------------
    println!("[7/7] Ingesting data into SQL Server...");

    // Change this slice to skip tables that are already populated.
    // For example, to insert only the fact table:
    //   &[DataMartTable::Fact]
    // To insert everything:
    //   &[
    //       DataMartTable::DimTime,
    //       DataMartTable::DimPersonAge,
    //       DataMartTable::DimPersonPosition,
    //       DataMartTable::DimPersonRole,
    //       DataMartTable::DimPersonSex,
    //       DataMartTable::DimPersonType,
    //       DataMartTable::DimContributingFactor,
    //       DataMartTable::Fact,
    //   ]
    let tables_to_ingest: &[DataMartTable] = &[
        DataMartTable::DimTime,
        DataMartTable::DimPersonAge,
        DataMartTable::DimPersonPosition,
        DataMartTable::DimPersonRole,
        DataMartTable::DimPersonSex,
        DataMartTable::DimPersonType,
        DataMartTable::DimContributingFactor,
        DataMartTable::Fact,
    ];

    let data_mart = DataMart {
        dim_time: &dm_times,
        dim_person_age: &dim_ages,
        dim_person_position: &dim_positions,
        dim_person_role: &dim_roles,
        dim_person_sex: &dim_sexes,
        dim_person_type: &dim_types,
        dim_contributing_factor: &dim_factors,
        fact: &facts,
    };

    if let Err(e) = datawarehousing_example_nyc_vehicle_incidents::ingestion::ingest_data_mart(
        &creds,
        &data_mart,
        tables_to_ingest,
    )
    .await
    {
        eprintln!("      ERROR during ingestion: {e:#}");
        return;
    }

    println!("Done.");
}

fn write_json<T: serde::Serialize>(path: &str, data: &T) {
    let json = serde_json::to_string_pretty(data).expect("failed to serialize to JSON");
    fs::write(path, json).unwrap_or_else(|e| panic!("failed to write {path}: {e}"));
    println!("      wrote {path}");
}

/// Reads database credentials from environment variables.
///
/// Required variables:
///   DB_USERNAME  – HTWK domain username (without the domain prefix)
///   DB_PASSWORD  – corresponding password
///
/// Optional overrides (defaults match the project server):
///   DB_HOST      – default: fimn-db1.htwk-leipzig.de
///   DB_PORT      – default: 1433
///   DB_DATABASE  – default: DWH25-04
///   DB_DOMAIN    – default: HTWK
fn db_credentials_from_env() -> DbCredentials {
    let username = std::env::var("DB_USERNAME").unwrap_or_else(|_| {
        eprintln!("WARNING: DB_USERNAME not set; connection will likely fail.");
        String::new()
    });
    let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
        eprintln!("WARNING: DB_PASSWORD not set; connection will likely fail.");
        String::new()
    });

    DbCredentials {
        host: std::env::var("DB_HOST").unwrap_or_else(|_| "fimn-db1.htwk-leipzig.de".into()),
        port: std::env::var("DB_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1433),
        database: std::env::var("DB_DATABASE").unwrap_or_else(|_| "DWH25-04".into()),
        domain: std::env::var("DB_DOMAIN").unwrap_or_else(|_| "HTWK".into()),
        username,
        password,
    }
}
