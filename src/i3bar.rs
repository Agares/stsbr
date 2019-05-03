use crate::data_source::DataSource;
use serde::Serialize;
use std::borrow::ToOwned;
use std::string::ToString;

#[derive(Serialize)]
struct Header {
    version: u64,
    click_events: bool,
}

#[derive(Serialize)]
struct Block {
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

fn convert_sources_to_blocks(sources: &Vec<&DataSource>) -> Vec<Block> {
    sources
        .iter()
        .map(|block| {
            block.current_state()
        })
        .filter_map(|state| {
            match state {
                Ok(st) => Some(Block {
                    full_text: st.text().to_owned(),
                    markup: "pango".to_string(),
                    instance: "fixme".to_string(),
                    name: "blahblahblahfixme".to_owned(),
                }),
                Err(e) => {
                    warn!("{}", e);
                    None
                }
            }
        })
        .collect::<Vec<Block>>()
}

pub fn sources_to_json(sources: &Vec<&DataSource>) -> String {
    let blocks = convert_sources_to_blocks(sources);

    serde_json::to_string(&blocks).unwrap()
}
