use crate::block::{Block, ClickEvent, Dimensions, MouseButton, Position};
use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;
use std::string::ToString;

#[derive(Serialize)]
struct Header {
    version: u64,
    click_events: bool,
}

#[derive(Serialize)]
struct BarBlock {
    full_text: String,
    markup: String,
    instance: String,
}

#[derive(Deserialize)]
struct Event {
    #[allow(unused)]
    instance: String,
    #[allow(unused)]
    button: u32,
    #[allow(unused)]
    x: u32,
    #[allow(unused)]
    y: u32,
    #[allow(unused)]
    relative_x: u32,
    #[allow(unused)]
    relative_y: u32,
    #[allow(unused)]
    width: u32,
    #[allow(unused)]
    height: u32,
}

pub fn get_header_json(allow_click_events: bool) -> String {
    let header = Header {
        version: 1,
        click_events: allow_click_events,
    };

    return serde_json::to_string(&header).unwrap();
}

fn convert_blocks_to_bar_blocks(sources: &Vec<Box<Block>>) -> Vec<BarBlock> {
    let mut bar_blocks = vec![];

    for i in 0..sources.len() {
        let source = &sources[i];
        let state = source.current_state();

        match state {
            Ok(st) => bar_blocks.push(BarBlock {
                full_text: st.text().to_owned(),
                markup: "pango".to_string(),
                instance: format!("{}", i),
            }),
            Err(e) => {
                warn!("{}", e);
            }
        }
    }

    bar_blocks
}

pub fn sources_to_json(sources: &Vec<Box<Block>>) -> String {
    let blocks = convert_blocks_to_bar_blocks(sources);

    serde_json::to_string(&blocks).unwrap()
}

pub fn read_event(raw: &str) -> ClickEvent {
    let raw_event = serde_json::from_str::<Event>(&raw).unwrap();

    let button = match raw_event.button {
        1 => MouseButton::Left,
        2 => MouseButton::Middle,
        3 => MouseButton::Right,
        4 => MouseButton::ScrollUp,
        5 => MouseButton::ScrollDown,
        _ => {
            warn!("Unknown mouse button {}", raw_event.button);
            MouseButton::Left
        }
    };

    let instance = match raw_event.instance.parse() {
        Ok(value) => value,
        Err(e) => panic!("Invalid instance ID: {}", e),
    };

    ClickEvent::new(
        button,
        Position(raw_event.relative_x, raw_event.relative_y),
        Dimensions(raw_event.width, raw_event.height),
        instance,
    )
}
