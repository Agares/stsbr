use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::char;

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
    fn current_state(&mut self) -> Result<BlockState, BlockError>;
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

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Icon {
    LightningBolt = 0xf0e7,
    VolumeUp = 0xf028,
    VolumeDown = 0xf027,
    VolumeOff = 0xf026,
    VolumeMute = 0xf6a9,
    Globe = 0xf0ac,
    Music = 0xf001,
    Play = 0xf04b,
    Pause = 0xf04c,
    HDD = 0xf0a0,
    Calendar = 0xf133
}

impl Display for Icon {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", char::from_u32(self.clone() as u32).unwrap())
    }
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
    pub fn button(&self) -> MouseButton {
        self.button
    }
}
