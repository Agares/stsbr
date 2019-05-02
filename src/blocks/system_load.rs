use crate::data_source::{DataSource, DataSourceState};
use std::os::raw::c_double;

pub struct SystemLoad {}

impl DataSource for SystemLoad {
    fn current_state(&self) -> DataSourceState {
        let mut load_averages: [c_double; 1] = [0f64];
        let received = unsafe { libc::getloadavg(load_averages.as_mut_ptr(), 1) };

        if received != 1 {
            // todo return Err()
            panic!("Cannot get load average!");
        }

        let load = load_averages[0];

        DataSourceState::new(format!("{}", load))
    }
}

impl SystemLoad {
    pub fn new() -> SystemLoad {
        SystemLoad {}
    }
}