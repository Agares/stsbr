use crate::block::{BlockError, Block, DataSourceState};

pub struct FreeDiskSpace {}

impl Block for FreeDiskSpace {
    fn current_state(&self) -> Result<DataSourceState, BlockError> {
        let stats = nix::sys::statvfs::statvfs("/")
            .map_err(|_| BlockError::new("Failed to stat".to_string()))?;

        let bytes_free = stats.blocks_available() * stats.block_size();
        let gigabytes_free = bytes_free as f32 / (1024.0f32.powi(3));

        Ok(DataSourceState::new(format!("{:.2} GB", gigabytes_free)))
    }
}

impl FreeDiskSpace {
    pub fn new() -> Self {
        FreeDiskSpace {}
    }
}
