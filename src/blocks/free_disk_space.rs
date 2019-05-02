use crate::data_source::{DataSource, DataSourceState};

pub struct FreeDiskSpace {}

impl DataSource for FreeDiskSpace {
    fn current_state(&self) -> DataSourceState {
        let stats = nix::sys::statvfs::statvfs("/").unwrap();

        let bytes_free = stats.blocks_available() * stats.block_size();
        let gigabytes_free = bytes_free as f32 / (1024.0f32.powi(3));
        DataSourceState::new(format!("{:.2} GB", gigabytes_free))
    }
}

impl FreeDiskSpace {
    pub fn new() -> Self {
        FreeDiskSpace {}
    }
}