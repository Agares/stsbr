use crate::block::{Block, BlockError, BlockState, ClickEvent, Icon};
use chrono::Local;

pub struct DateTime {}

impl Block for DateTime {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let now: chrono::DateTime<Local> = Local::now();

        Ok(BlockState::new(format!("{} {}", Icon::Calendar, now.format("%Y-%m-%d %T"))))
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime {}
    }
}
