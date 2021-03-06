use crate::block::Block;
use crate::blocks::date_time::DateTime;
use crate::blocks::free_disk_space::FreeDiskSpace;
use crate::blocks::media_player::MediaPlayer;
use crate::blocks::network_interface::NetworkInterface;
use crate::blocks::system_load::SystemLoad;
use crate::blocks::volume::VolumeFactory;
use crate::i3bar::{get_header_json, read_event, sources_to_json};
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use toml::Value;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod block;
mod blocks;
mod i3bar;

fn main() {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("stsbr.log").unwrap(),
    )
    .unwrap();

    let mut sources = load_blocks();
    let receiver = create_stdin_thread();

    println!("{}", get_header_json(true));
    println!("[");

    loop {
        println!("{},", sources_to_json(&mut sources));

        while let Ok(x) = receiver.try_recv() {
            if x != "[\n" {
                let event = read_event(x.trim_matches(','));
                let block = sources.get(event.instance()).unwrap();

                block.handle_click(event);
            }
        }

        std::thread::sleep(Duration::from_millis(200));
    }
}

fn load_blocks() -> Vec<Box<Block>> {
    let mut config = String::new();

    File::open(".stsbr.toml")
        .unwrap()
        .read_to_string(&mut config)
        .unwrap();

    let block_factories = create_block_factories();
    let mut sources: Vec<Box<dyn Block>> = vec![];

    parse_config(
        &config,
        Box::new(|section| {
            let module_name = section["module"].as_str().unwrap();

            sources.push(block_factories[module_name](section));
        }),
    );
    sources
}

fn parse_config<'a>(config: &str, mut on_section: Box<'a + FnMut(&Value)>) {
    let parsed_config = config.parse::<Value>().unwrap();

    if let Value::Table(i) = parsed_config {
        if let Value::Table(toml_sources) = i.get("sources").unwrap() {
            for (_, section) in toml_sources.iter().rev() {
                on_section(&section);
            }
        }
    }
}

type BlockFactory = Fn(&Value) -> Box<Block>;
fn create_block_factories() -> HashMap<String, Box<BlockFactory>> {
    let volume_factory = Rc::new(VolumeFactory::new());

    let mut block_factories: HashMap<String, Box<BlockFactory>> = HashMap::new();

    block_factories.insert("date_time".into(), Box::new(|_| Box::new(DateTime::new())));
    block_factories.insert(
        "free_disk_space".into(),
        Box::new(|_| Box::new(FreeDiskSpace::new())),
    );
    block_factories.insert(
        "media_player".into(),
        Box::new(|_| Box::new(MediaPlayer::new())),
    );
    block_factories.insert(
        "network_interface".into(),
        Box::new(|section| {
            Box::new(NetworkInterface::new(
                section["interface"].as_str().unwrap().into(),
            ))
        }),
    );
    block_factories.insert(
        "system_load".into(),
        Box::new(|_| Box::new(SystemLoad::new())),
    );
    block_factories.insert(
        "volume".into(),
        Box::new(move |section| {
            if let Value::Table(x) = section {
                let volume = volume_factory
                    .clone()
                    .new_volume(x["sink"].as_str().unwrap().into());

                Box::new(volume)
            } else {
                panic!("Invalid configuration file");
            }
        }),
    );

    block_factories
}

fn create_stdin_thread() -> Receiver<String> {
    let (sender, receiver): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let stdin = std::io::stdin();

        loop {
            let mut line = String::new();

            match stdin.read_line(&mut line) {
                Ok(_) => sender.send(line).unwrap(),
                Err(e) => {
                    error!("{}", e);
                    break;
                }
            };
        }
    });

    receiver
}
