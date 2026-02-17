use time::{Date, Time, macros::format_description};

#[derive(Debug, Clone)]
pub struct RawCrashRecord {
    pub collision_id: u32,
    pub crash_date: Date,
    pub crash_time: Time,
    pub number_of_persons_injured: u16,
    pub number_of_persons_killed: u16,
    pub number_of_pedestrians_injured: u16,
    pub number_of_pedestrians_killed: u16,
    pub number_of_cyclist_injured: u16,
    pub number_of_cyclist_killed: u16,
    pub number_of_motorist_injured: u16,
    pub number_of_motorist_killed: u16,
    pub contributing_factor_vehicle_1: String,
    pub contributing_factor_vehicle_2: String,
    pub contributing_factor_vehicle_3: String,
    pub contributing_factor_vehicle_4: String,
    pub contributing_factor_vehicle_5: String,
}

impl RawCrashRecord {
    pub fn load_from_csv(path: &str) -> Vec<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(path)
            .unwrap();
        let mut records = Vec::new();
        let date_fmt = format_description!("[month]/[day]/[year]");
        let time_fmt = format_description!("[hour padding:none]:[minute]");

        for result in rdr.records() {
            let string_record = result.unwrap();
            let record = RawCrashRecord {
                collision_id: string_record[23].parse().unwrap(),
                crash_date: time::Date::parse(&string_record[0], &date_fmt).unwrap(),
                crash_time: time::Time::parse(&string_record[1], &time_fmt).unwrap(),
                number_of_persons_injured: string_record[10].parse().unwrap_or(0),
                number_of_persons_killed: string_record[11].parse().unwrap_or(0),
                number_of_pedestrians_injured: string_record[12].parse().unwrap_or(0),
                number_of_pedestrians_killed: string_record[13].parse().unwrap_or(0),
                number_of_cyclist_injured: string_record[14].parse().unwrap_or(0),
                number_of_cyclist_killed: string_record[15].parse().unwrap_or(0),
                number_of_motorist_injured: string_record[16].parse().unwrap_or(0),
                number_of_motorist_killed: string_record[17].parse().unwrap_or(0),
                contributing_factor_vehicle_1: string_record[18].to_string(),
                contributing_factor_vehicle_2: string_record[19].to_string(),
                contributing_factor_vehicle_3: string_record[20].to_string(),
                contributing_factor_vehicle_4: string_record[21].to_string(),
                contributing_factor_vehicle_5: string_record[22].to_string(),
            };
            records.push(record);
        }
        records
    }
}
