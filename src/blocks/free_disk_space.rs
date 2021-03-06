use crate::block::{Block, BlockError, BlockState, ClickEvent, Icon};

pub struct FreeDiskSpace {}

impl Block for FreeDiskSpace {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let stats = nix::sys::statvfs::statvfs("/")
            .map_err(|_| BlockError::new("Failed to stat".to_string()))?;

        let bytes_free = stats.blocks_available() * stats.block_size();
        let gigabytes_free = bytes_free as f32 / (1024.0f32.powi(3));

        Ok(BlockState::new(format!(
            "{} {:.2} GB",
            Icon::HDD,
            gigabytes_free
        )))
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl FreeDiskSpace {
    pub fn new() -> Self {
        FreeDiskSpace {}
    }
}
