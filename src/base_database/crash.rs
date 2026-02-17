use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct Crash {
    pub crash_id: u32,
    pub crash_timestamp: OffsetDateTime,
    pub crash_persons_injured: u16,
    pub crash_persons_killed: u16,
    pub crash_pedestrians_injured: u16,
    pub crash_pedestrians_killed: u16,
    pub crash_cyclist_injured: u16,
    pub crash_cyclist_killed: u16,
    pub crash_motorist_injured: u16,
    pub crash_motorist_killed: u16,
    pub crash_contributing_factor_vehicle_1: String,
    pub crash_contributing_factor_vehicle_2: String,
    pub crash_contributing_factor_vehicle_3: String,
    pub crash_contributing_factor_vehicle_4: String,
    pub crash_contributing_factor_vehicle_5: String,
}
