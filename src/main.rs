use crate::data_source::DataSource;
use crate::i3bar::{get_header_json, sources_to_json};
use std::time::Duration;
use libpulse_binding::mainloop::threaded::Mainloop;
use libpulse_binding::context::Context;
use crate::blocks::date_time::DateTime;
use crate::blocks::system_load::SystemLoad;
use crate::blocks::network_interface::NetworkInterface;
use crate::blocks::free_disk_space::FreeDiskSpace;
use crate::blocks::media_player::MediaPlayer;
use crate::blocks::volume::V;

#[macro_use]
extern crate lazy_static;

mod data_source;
mod i3bar;
mod blocks;

fn main() {
    let date_time = DateTime::new();
    let system_load = SystemLoad::new();
    let network_interface = NetworkInterface::new();
    let free_disk_space = FreeDiskSpace::new();
    let media_player = MediaPlayer::new();

    let sources: Vec<&DataSource> = vec![
        &(*V),
        &media_player,
        &free_disk_space,
        &network_interface,
        &system_load,
        &date_time,
    ];

    let mut mainloop = Mainloop::new().unwrap();
    mainloop.start().unwrap();

    let mut ctx = Context::new(&mainloop, "stsbr").unwrap();
    ctx.connect(None, libpulse_binding::context::flags::NOFLAGS, None).unwrap();

    loop {
        match ctx.get_state() {
            libpulse_binding::context::State::Ready => {break;},
            libpulse_binding::context::State::Failed
            | libpulse_binding::context::State::Terminated => {
                panic!("Failed to connect to pulse");
            },
            _ => {}
        }
    }

    println!("{}", get_header_json(false));
    println!("[");

    loop {
        ctx.introspect().get_sink_info_by_name("@DEFAULT_SINK@", |info| V.callback_sink_info(info));
        println!("{},", sources_to_json(&sources));

        std::thread::sleep(Duration::from_millis(200));
    }
}
