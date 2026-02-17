use time::{Date, Time, macros::format_description};

/// https://dev.socrata.com/foundry/data.cityofnewyork.us/f55k-p6yu
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawPersonRecord {
    pub unique_id: String,
    pub collision_id: u32,
    pub crash_date: Date,
    pub crash_time: Time,
    pub person_type: Option<String>,
    pub person_age: Option<u8>,
    pub person_sex: Option<char>,
    pub person_position_in_vehicle: Option<String>,
    pub person_ped_role: Option<String>,
}

impl RawPersonRecord {
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
            let record = RawPersonRecord {
                unique_id: string_record[0].into(),
                collision_id: string_record[1].parse().unwrap(),
                crash_date: time::Date::parse(&string_record[2], &date_fmt).unwrap(),
                crash_time: time::Time::parse(&string_record[3], &time_fmt).unwrap(),
                person_type: if string_record[5].is_empty() {
                    None
                } else {
                    Some(string_record[5].into())
                },
                person_age: string_record[8]
                    .parse::<u8>()
                    .map(Into::into)
                    .unwrap_or(None),
                person_sex: string_record[20].parse().ok(),
                person_position_in_vehicle: if string_record[12].is_empty() {
                    None
                } else {
                    Some(string_record[12].into())
                },
                person_ped_role: if string_record[17].is_empty() {
                    None
                } else {
                    Some(string_record[17].into())
                },
            };
            records.push(record);
        }
        records
    }
}
