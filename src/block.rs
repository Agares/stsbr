use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct BlockError(String);

impl Error for BlockError {}

impl Display for BlockError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.0)
    }
}

impl BlockError {
    pub fn new(msg: String) -> Self {
        BlockError(msg)
    }
}

pub trait Block {
    fn current_state(&self) -> Result<DataSourceState, BlockError>;
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
