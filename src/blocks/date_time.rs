use crate::data_source::{DataSource, DataSourceState, BlockError};
use chrono::Local;

pub struct DateTime {}

impl DataSource for DateTime {
    fn current_state(&self) -> Result<DataSourceState, BlockError> {
        let now: chrono::DateTime<Local> = Local::now();

        Ok(DataSourceState::new(now.format("%Y-%m-%d %T").to_string()))
    }
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime {}
    }
}