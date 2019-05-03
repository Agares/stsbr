use crate::data_source::Block;
use serde::Serialize;
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
    name: String,
    instance: String,
}

pub fn get_header_json(allow_click_events: bool) -> String {
    let header = Header {
        version: 1,
        click_events: allow_click_events,
    };

    return serde_json::to_string(&header).unwrap();
}

fn convert_blocks_to_bar_blocks(sources: &Vec<&Block>) -> Vec<BarBlock> {
    sources
        .iter()
        .map(|block| block.current_state())
        .filter_map(|state| match state {
            Ok(st) => Some(BarBlock {
                full_text: st.text().to_owned(),
                markup: "pango".to_string(),
                instance: "fixme".to_string(),
                name: "blahblahblahfixme".to_owned(),
            }),
            Err(e) => {
                warn!("{}", e);
                None
            }
        })
        .collect::<Vec<BarBlock>>()
}

pub fn sources_to_json(sources: &Vec<&Block>) -> String {
    let blocks = convert_blocks_to_bar_blocks(sources);

    serde_json::to_string(&blocks).unwrap()
}
