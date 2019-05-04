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
    fn current_state(&self) -> Result<BlockState, BlockError>;
    fn handle_click(&self, event: ClickEvent);
}

pub struct BlockState {
    text: String,
}

impl BlockState {
    pub fn new(text: String) -> BlockState {
        BlockState { text }
    }

    pub fn text(&self) -> &String {
        return &self.text;
    }
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub struct Position(pub u32, pub u32);
#[derive(Debug)]
pub struct Dimensions(pub u32, pub u32);

#[derive(Debug)]
pub struct ClickEvent {
    button: MouseButton,
    position: Position,
    block_dimensions: Dimensions,
    instance: usize,
}

impl ClickEvent {
    pub fn new(
        button: MouseButton,
        position: Position,
        block_dimensions: Dimensions,
        instance: usize,
    ) -> Self {
        ClickEvent {
            button,
            position,
            block_dimensions,
            instance,
        }
    }

    pub fn instance(&self) -> usize {
        self.instance
    }
}
