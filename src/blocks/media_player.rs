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
    current_state: String,
}

impl Block for MediaPlayer {
    fn current_state(&mut self) -> Result<BlockState, BlockError> {
        let state = self.state_receiver.try_iter().last();

        if let Some(new_state) = state {
            match new_state {
                MediaPlayerStateChange::NowPlaying { artist, title } => {
                    self.current_state = format!("{} {} - {}", Icon::Music, artist, title);
                }
            }
        };

        if self.current_state != "" {
            Ok(BlockState::new(self.current_state.clone()))
        } else {
            Err(BlockError::new("Unknown state".into()))
        }
    }

    fn handle_click(&self, _event: ClickEvent) {
        self.command_sender
            .send(MediaPlayerRequest::TogglePause)
            .unwrap();
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

#[derive(Debug)]
struct PlayerError(String);

impl From<DBusError> for PlayerError {
    fn from(err: DBusError) -> Self {
        PlayerError(format!("{:?}", err))
    }
}

fn get_artist_and_title(player:&Player) -> Result<Option<(String, String)>, PlayerError> {
    let metadata = player.get_metadata()?;
    let artists;
    let title;

    if let Some(artists_vec) = metadata.artists() {
        artists = artists_vec.join(", ");
    } else {
        return Ok(None);
    }

    if let Some(title_opt) = metadata.title() {
        title = title_opt.into();
    } else {
        return Ok(None);
    }

    Ok(Some((artists, title)))
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

                let should_continue:Result<bool, PlayerError> = find_player()
                    .map(|player| {
                        let artist_and_title = get_artist_and_title(&player)?;

                        if let Some((artist, title)) = artist_and_title {
                            state_sender
                                .send(MediaPlayerStateChange::NowPlaying {
                                    artist,
                                    title,
                                })
                                .unwrap();
                        }

                        match message {
                            Result::Ok(MediaPlayerRequest::Quit) => Ok(false),
                            Result::Ok(MediaPlayerRequest::TogglePause) => {
                                player.play_pause().unwrap();
                                Ok(true)
                            }
                            Result::Err(_) => Ok(true),
                        }
                    })
                    .unwrap_or(Ok(true));

                if let Ok(false) = should_continue {
                    break 'mainloop;
                }

                // fixme log error
            })),
            command_sender,
            state_receiver,
            current_state: "".into(),
        }
    }
}

impl Drop for MediaPlayer {
    fn drop(&mut self) {
        self.command_sender.send(MediaPlayerRequest::Quit).unwrap();
        self.thread.take().unwrap().join().unwrap();
    }
}
