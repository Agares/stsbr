use crate::block::{Block, BlockError, BlockState, ClickEvent, Icon};
use std::os::raw::c_double;

pub struct SystemLoad {}

impl Block for SystemLoad {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let mut load_averages: [c_double; 1] = [0f64];
        let received = unsafe { libc::getloadavg(load_averages.as_mut_ptr(), 1) };

        if received != 1 {
            return Err(BlockError::new("Cannot get load average!".to_string()));
        }

        let load = load_averages[0];

        Ok(BlockState::new(format!("{} {:.2}", Icon::LightningBolt, load)))
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl SystemLoad {
    pub fn new() -> SystemLoad {
        SystemLoad {}
    }
}
