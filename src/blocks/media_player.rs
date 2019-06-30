use crate::block::{Block, BlockError, BlockState, ClickEvent, Icon};
use mpris::{DBusError, FindingError, Player, PlayerFinder};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

enum MediaPlayerRequest {
    Quit,
    TogglePause,
}

enum MediaPlayerStateChange {
    NowPlaying { artist: String, title: String },
}

pub struct MediaPlayer {
    thread: Option<JoinHandle<()>>,
    command_sender: Sender<MediaPlayerRequest>,
    state_receiver: Receiver<MediaPlayerStateChange>,
    current_state: String
}

impl Block for MediaPlayer {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let state = self.state_receiver.try_iter().last();

        match state {
            Some(new_state) => match new_state {
                MediaPlayerStateChange::NowPlaying { artist, title } => {
                    self.current_state = format!("{} {} - {}", Icon::Music, artist, title);
                }
            },
            None => {}
        };

        if self.current_state != "" {
            Ok(BlockState::new(self.current_state.clone()))
        } else {
            Err(BlockError::new("Unknown state".into()))
        }
    }

    fn handle_click(&self, _event: ClickEvent) {
        self.command_sender.send(MediaPlayerRequest::TogglePause).unwrap();
    }
}

#[derive(Debug)]
struct PlayerFindingError(String);

impl From<DBusError> for PlayerFindingError {
    fn from(err: DBusError) -> Self {
        PlayerFindingError(format!("{:?}", err))
    }
}

impl From<FindingError> for PlayerFindingError {
    fn from(err: FindingError) -> Self {
        PlayerFindingError(format!("{:?}", err))
    }
}

impl From<PlayerFindingError> for BlockError {
    fn from(err: PlayerFindingError) -> Self {
        BlockError::new(format!("{:?}", err))
    }
}

fn find_player<'a>() -> Result<Player<'a>, PlayerFindingError> {
    let player = PlayerFinder::new()?.find_active()?;

    Ok(player)
}

impl MediaPlayer {
    pub fn new() -> Self {
        let (command_sender, command_receiver): (
            Sender<MediaPlayerRequest>,
            Receiver<MediaPlayerRequest>,
        ) = std::sync::mpsc::channel();

        let (state_sender, state_receiver): (
            Sender<MediaPlayerStateChange>,
            Receiver<MediaPlayerStateChange>,
        ) = std::sync::mpsc::channel();

        MediaPlayer {
            thread: Some(thread::spawn(move || 'mainloop: loop {
                let message = command_receiver.recv_timeout(Duration::from_millis(500));

                let should_continue = find_player().map(|player| {
                    let metadata = player
                        .get_metadata()
                        .or_else(|_| Err(BlockError::new("".into())));

                    metadata.map(|metadata| {
                        let artist = metadata.artists().and_then(|artists| artists.first());

                        let title = metadata.title();

                        match (artist, title) {
                            (Some(artist), Some(title)) => {
                                state_sender.send(MediaPlayerStateChange::NowPlaying {
                                    artist: artist.clone(),
                                    title: title.into(),
                                }).unwrap();
                            }
                            _ => {}
                        }
                    }).unwrap();

                    match message {
                        Result::Ok(MediaPlayerRequest::Quit) => false,
                        Result::Ok(MediaPlayerRequest::TogglePause) => {
                            player.play_pause().unwrap();
                            true
                        }
                        Result::Err(_) => true,
                    }
                }).unwrap_or(true);

                if !should_continue {
                    break 'mainloop
                }
            })),
            command_sender,
            state_receiver,
            current_state: "".into()
        }
    }
}

impl Drop for MediaPlayer {
    fn drop(&mut self) {
        self.command_sender.send(MediaPlayerRequest::Quit).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}
