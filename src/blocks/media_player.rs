use crate::data_source::{DataSourceState, DataSource};

pub struct MediaPlayer {}

impl DataSource for MediaPlayer {
    fn current_state(&self) -> DataSourceState {
        let bus = mpris::PlayerFinder::new().unwrap();
        let all_players = bus.find_all().unwrap();
        let player = all_players.first().unwrap();
        let metadata = player.get_metadata().unwrap();
        let formatted = format!(
            "{} - {}",
            metadata.artists().unwrap().first().unwrap(),
            metadata.title().unwrap()
        );

        DataSourceState::new(formatted)
    }
}

impl MediaPlayer {
    pub fn new() -> Self {
        MediaPlayer {}
    }
}