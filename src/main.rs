use crate::data_source::{DataSource, DataSourceState};
use crate::i3bar::{get_header_json, sources_to_json};
use chrono::Local;
use std::time::Duration;

mod data_source;
mod i3bar;

struct DateTime {}
impl DataSource for DateTime {
    fn current_state(&self) -> DataSourceState {
        let now: chrono::DateTime<Local> = Local::now();
        DataSourceState::new(now.format("%Y-%m-%d %T").to_string())
    }
}

fn main() {
    let sources: Vec<Box<DataSource>> = vec![
        Box::new(DateTime {}),
        Box::new(DateTime {})
    ];

    println!("{}", get_header_json(false));
    println!("[");

    loop {
        println!("{},", sources_to_json(&sources));

        std::thread::sleep(Duration::from_millis(200));
    }
}
