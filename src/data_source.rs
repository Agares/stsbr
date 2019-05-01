pub trait DataSource {
    fn current_state(&self) -> DataSourceState;
}

pub struct DataSourceState {
    text: String,
}

impl DataSourceState {
    pub fn new(text: String) -> DataSourceState {
        DataSourceState { text }
    }

    pub fn text(&self) -> &String {
        return &self.text;
    }
}
