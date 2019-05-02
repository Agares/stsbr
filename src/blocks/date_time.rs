use crate::data_source::{DataSource, DataSourceState};
use chrono::Local;

pub struct DateTime {}

impl DataSource for DateTime {
    fn current_state(&self) -> DataSourceState {
        let now: chrono::DateTime<Local> = Local::now();
        DataSourceState::new(now.format("%Y-%m-%d %T").to_string())
    }
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime {}
    }
}