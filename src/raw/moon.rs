use time::{Date, macros::format_description};

/// https://www.kaggle.com/datasets/jodiemullins/1900-2022-primary-moon-phases-utc7-timezone
#[derive(Debug, Clone)]
pub struct RawMoonRecord {
    pub new_moon: Option<Date>,
    pub first_quarter_moon: Option<Date>,
    pub full_moon: Option<Date>,
    pub third_quarter_moon: Option<Date>,
}

impl RawMoonRecord {
    pub fn load_from_csv(path: &str) -> Vec<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(path)
            .unwrap();
        let mut records = Vec::new();
        let date_fmt = format_description!("[month padding:none]/[day padding:none]/[year]");

        for result in rdr.records() {
            let string_record = result.unwrap();
            let record = RawMoonRecord {
                new_moon: time::Date::parse(&string_record[0], &date_fmt).ok(),
                first_quarter_moon: time::Date::parse(&string_record[1], &date_fmt).ok(),
                full_moon: time::Date::parse(&string_record[2], &date_fmt).ok(),
                third_quarter_moon: time::Date::parse(&string_record[3], &date_fmt).ok(),
            };
            records.push(record);
        }
        records
    }
}
