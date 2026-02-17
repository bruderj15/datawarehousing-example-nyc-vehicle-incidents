use time::{OffsetDateTime, format_description::well_known};

/// https://www.kaggle.com/datasets/aadimator/nyc-weather-2016-to-2022
#[derive(Debug, Clone)]
pub struct RawWeatherRecord {
    pub time: OffsetDateTime,
    pub temperature: f32,
    pub precipitation: f32,
    pub rain: f32,
}

impl RawWeatherRecord {
    pub fn load_from_csv(path: &str) -> Vec<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(path)
            .unwrap();
        let mut records = Vec::new();
        for result in rdr.records() {
            let string_record = result.unwrap();
            let record = RawWeatherRecord {
                time: time::OffsetDateTime::parse(&string_record[0], &well_known::Rfc3339).unwrap(),
                temperature: string_record[1].parse().unwrap(),
                precipitation: string_record[2].parse().unwrap(),
                rain: string_record[3].parse().unwrap(),
            };
            records.push(record);
        }
        records
    }
}
