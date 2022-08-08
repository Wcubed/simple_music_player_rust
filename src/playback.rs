use libmpv::{FileState, Mpv, MpvNode};
use log::{error, info, warn};
use std::path::Path;

const PROP_VOLUME: &str = "volume";
const PROP_PAUSE: &str = "pause";
const PROP_VIDEO_OUTPUT: &str = "vo";
const PROP_PLAYBACK_TIME: &str = "playback-time";
const PROP_SONG_DURATION: &str = "duration";

pub struct Playback {
    mpv: Mpv,
}

impl Playback {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn play_file(&self, path: &Path) -> libmpv::Result<()> {
        self.mpv
            .playlist_load_files(&[(&path.to_string_lossy(), FileState::Replace, None)])
    }

    pub fn is_paused(&self) -> bool {
        self.mpv.get_property(PROP_PAUSE).unwrap_or(true)
    }

    pub fn pause(&self) {
        match self.mpv.pause() {
            Ok(_) => {}
            Err(e) => warn!("Could not pause: {}", e),
        }

        let state: MpvNode = self.mpv.get_property("demuxer-cache-state").unwrap();
        info!("{:?}", state);
    }

    pub fn unpause(&self) {
        match self.mpv.unpause() {
            Ok(_) => {}
            Err(e) => warn!("Could not unpause: {}", e),
        }
    }

    pub fn volume(&self) -> i64 {
        self.mpv.get_property(PROP_VOLUME).unwrap_or(0)
    }

    pub fn set_volume(&self, volume: i64) {
        self.mpv
            .set_property(PROP_VOLUME, volume)
            .expect("Could not set volume");
    }

    ///  how much of the song has been played.
    pub fn current_song_seconds_played(&self) -> u64 {
        self.mpv.get_property(PROP_PLAYBACK_TIME).unwrap_or(0) as u64
    }

    pub fn current_song_length_in_seconds(&self) -> u64 {
        self.mpv.get_property(PROP_SONG_DURATION).unwrap_or(0) as u64
    }
}

impl Default for Playback {
    /// Will fail if the mpv library object cannot be created.
    fn default() -> Self {
        let mpv = Mpv::new().expect("Could not create mpv library instance.");

        // Start paused.
        mpv.pause().unwrap();

        // Don't show any video output.
        // Prevents mpv from showing an album or song image if it sees one.
        mpv.set_property(PROP_VIDEO_OUTPUT, "null").unwrap();

        Playback { mpv }
    }
}
