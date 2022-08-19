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
}
