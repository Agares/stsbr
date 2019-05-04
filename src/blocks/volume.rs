use crate::block::{Block, BlockError, BlockState, ClickEvent, MouseButton};
use libpulse_binding::callbacks::ListResult::Item;
use libpulse_binding::context::Context;
use libpulse_binding::mainloop::threaded::Mainloop;
use libpulse_binding::volume::{ChannelVolumes, VolumeDB, VolumeLinear, VOLUME_MUTED};
use std::collections::HashMap;
use std::io::sink;
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

struct SinkInfo {
    current_volume: String,
    channels: u8,
    volume: u32,
    muted: bool,
}

lazy_static! {
    static ref SINK_VOLUME: Mutex<HashMap<String, SinkInfo>> = Mutex::new(HashMap::new());
}

impl<'a> Block for Volume<'a> {
    fn current_state(&self) -> Result<BlockState, BlockError> {
        let sink_name = self.sink_name.clone();
        self.context
            .introspect()
            .get_sink_info_by_name(&self.sink_name, move |info| match info {
                Item(sink_info) => {
                    let mut volume_map = SINK_VOLUME.lock().unwrap();
                    let linear = (*sink_info).volume.avg();

                    volume_map.insert(
                        sink_name.clone(),
                        SinkInfo {
                            volume: linear.0,
                            current_volume: (*sink_info).volume.avg().print(),
                            channels: (*sink_info).volume.channels,
                            muted: (*sink_info).mute,
                        },
                    );
                }
                _ => {}
            });

        let guard = SINK_VOLUME.lock().unwrap();
        let sink_volume = guard.get(&self.sink_name);

        match sink_volume {
            Some(i) => Ok(BlockState::new(i.current_volume.clone())),
            None => Err(BlockError::new("Unknown volume".to_string())),
        }
    }

    fn handle_click(&self, event: ClickEvent) {
        let guard = SINK_VOLUME.lock().unwrap();
        let sink_volume = guard.get(&self.sink_name).unwrap();

        if let MouseButton::Left = event.button() {
            self.context.introspect().set_sink_mute_by_name(
                &self.sink_name,
                !sink_volume.muted,
                Some(Box::new(|st| info!("{:?}", st))),
            );

            return;
        }

        let step = (libpulse_binding::volume::VOLUME_NORM.0
            - libpulse_binding::volume::VOLUME_MUTED.0)
            / 20;
        let final_volume = match event.button() {
            MouseButton::ScrollUp => sink_volume.volume.saturating_add(step),
            MouseButton::ScrollDown => sink_volume.volume.saturating_sub(step),
            _ => sink_volume.volume,
        };

        let mut channelvolums = ChannelVolumes {
            channels: 2,
            values: [libpulse_binding::volume::Volume(final_volume); 32],
        };
        self.context
            .introspect()
            .set_sink_volume_by_name(&self.sink_name, &channelvolums, None);
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
