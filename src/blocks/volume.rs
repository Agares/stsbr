use std::string::ToString;
use std::sync::Mutex;
use crate::data_source::{DataSource, DataSourceState, BlockError};
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::introspect::SinkInfo;
use libpulse_binding::callbacks::ListResult::Item;

lazy_static! {
    pub static ref V:Volume = Volume::new();
}

pub struct Volume {
    sink_info:Mutex<Option<String>>
}

impl DataSource for Volume {
    fn current_state(&self) -> Result<DataSourceState, BlockError> {
        let info;
        {
            let lock_result = self.sink_info.lock();

            match lock_result {
                Ok(i) => info = (*i).clone(),
                Err(_) => return Err(BlockError::new("Failed to lock on mutex".to_string()))
            }
        }

        if let Some(x) = info {
            Ok(DataSourceState::new(x))
        } else {
            Err(BlockError::new("Failed to get current volume".to_string()))
        }
    }
}

impl Volume {
    pub fn new() -> Self {
        Volume { sink_info: Mutex::new(None) }
    }

    pub fn callback_sink_info(&self, info:ListResult<&SinkInfo>) {
        match info {
            Item(x) => {
                let mut reference = self
                    .sink_info
                    .lock()
                    .unwrap();

                *reference = Some((*x).volume.print())
            }
            _ => {}
        }
    }
}