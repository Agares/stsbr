use crate::block::{BlockError, Block, BlockState};
use chrono::Local;

pub struct DateTime {}

impl Block for DateTime {
    fn current_state(&self) -> Result<BlockState, BlockError> {
        let now: chrono::DateTime<Local> = Local::now();

        Ok(BlockState::new(now.format("%Y-%m-%d %T").to_string()))
    }
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime {}
    }
}
