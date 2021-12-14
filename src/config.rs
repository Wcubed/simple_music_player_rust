use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub library_directory: PathBuf,
}
