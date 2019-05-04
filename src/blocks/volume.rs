use crate::block::{BlockError, Block, DataSourceState};
use libpulse_binding::callbacks::ListResult::Item;
use libpulse_binding::context::Context;
use libpulse_binding::mainloop::threaded::Mainloop;
use std::collections::HashMap;
use std::string::ToString;
use std::sync::Mutex;

pub struct VolumeFactory {
    context: Context,
    #[allow(unused)]
    main_loop: Mainloop,
}

pub struct Volume<'a> {
    sink_name: String,
    context: &'a Context,
}

lazy_static! {
    static ref SINK_VOLUME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

impl<'a> Block for Volume<'a> {
    fn current_state(&self) -> Result<DataSourceState, BlockError> {
        let sink_name = self.sink_name.clone();
        self.context
            .introspect()
            .get_sink_info_by_name(&self.sink_name, move |info| match info {
                Item(sink_info) => {
                    let mut volume_map = SINK_VOLUME.lock().unwrap();

                    volume_map.insert(sink_name.clone(), (*sink_info).volume.avg().print());
                }
                _ => {}
            });

        let guard = SINK_VOLUME.lock().unwrap();
        let sink_volume = guard.get(&self.sink_name);

        match sink_volume {
            Some(i) => Ok(DataSourceState::new(i.clone())),
            None => Err(BlockError::new("Unknown volume".to_string())),
        }
    }
}

impl VolumeFactory {
    pub fn new() -> Self {
        let mut mainloop = Mainloop::new().unwrap();
        mainloop.start().unwrap();

        let mut ctx = Context::new(&mainloop, "stsbr").unwrap();
        ctx.connect(None, libpulse_binding::context::flags::NOFLAGS, None)
            .unwrap();

        loop {
            match ctx.get_state() {
                libpulse_binding::context::State::Ready => {
                    break;
                }
                libpulse_binding::context::State::Failed
                | libpulse_binding::context::State::Terminated => {
                    panic!("Failed to connect to pulse");
                }
                _ => {}
            }
        }

        VolumeFactory {
            context: ctx,
            main_loop: mainloop,
        }
    }

    pub fn new_volume(&self, sink_name: String) -> Volume {
        Volume {
            sink_name,
            context: &self.context,
        }
    }
}
