use crate::block::{Block, BlockError, BlockState, ClickEvent};
use chrono::Local;

pub struct DateTime {}

impl Block for DateTime {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let now: chrono::DateTime<Local> = Local::now();

        Ok(BlockState::new(now.format("%Y-%m-%d %T").to_string()))
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime {}
    }
}
