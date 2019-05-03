use crate::blocks::date_time::DateTime;
use crate::blocks::free_disk_space::FreeDiskSpace;
use crate::blocks::media_player::MediaPlayer;
use crate::blocks::network_interface::NetworkInterface;
use crate::blocks::system_load::SystemLoad;
use crate::blocks::volume::VolumeFactory;
use crate::data_source::Block;
use crate::i3bar::{get_header_json, sources_to_json};
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod blocks;
mod data_source;
mod i3bar;

fn main() {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("stsbr.log").unwrap(),
    )
    .unwrap();

    let date_time = DateTime::new();
    let system_load = SystemLoad::new();
    let network_interface = NetworkInterface::new();
    let free_disk_space = FreeDiskSpace::new();
    let media_player = MediaPlayer::new();
    let volume_factory = VolumeFactory::new();
    let volume = volume_factory.new_volume("@DEFAULT_SINK@".into());

    let sources: Vec<&Block> = vec![
        &volume,
        &media_player,
        &free_disk_space,
        &network_interface,
        &system_load,
        &date_time,
    ];
    println!("{}", get_header_json(false));
    println!("[");

    loop {
        println!("{},", sources_to_json(&sources));

        std::thread::sleep(Duration::from_millis(200));
    }
}
