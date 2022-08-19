use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
// Auto fill properties with their defaults if they are missing.
// Allows properties to be added to future versions without breaking the configs.
#[serde(default)]
pub struct Config {
    pub library_directory: PathBuf,
    /// An infinite playlist automatically adds and removes songs when it reaches near the end.
    pub infinite_playlist: bool,
    /// How many songs an infinite playlist should keep in buffer in front of the current song.
    #[serde(default = "default_infinite_buffer")]
    pub infinite_playlist_song_buffer: u32,
    /// How many songs an infinite playlist should keep behind the currently playing song,
    /// before removing them from the playlist.
    #[serde(default = "default_infinite_buffer")]
    pub infinite_playlist_song_rear_buffer: u32,
}

fn default_infinite_buffer() -> u32 {
    4
}
