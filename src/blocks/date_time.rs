use crate::block::{BlockError, Block, DataSourceState};
use chrono::Local;

pub struct DateTime {}

impl Block for DateTime {
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
