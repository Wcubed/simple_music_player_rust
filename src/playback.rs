use libmpv::{FileState, Mpv};
use log::warn;
use std::path::Path;

const PROP_VOLUME: &str = "volume";
const PROP_PAUSE: &str = "pause";
const PROP_VIDEO_OUTPUT: &str = "vo";
const PROP_PLAYBACK_TIME: &str = "playback-time";
const PROP_SONG_DURATION: &str = "duration";
const PROP_PAUSE_WHEN_SONG_ENDS: &str = "keep-open";

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

    /// Will also return `true` when stopped at the end of a song.
    pub fn is_paused(&self) -> bool {
        self.mpv.get_property(PROP_PAUSE).unwrap_or(true)
    }

    pub fn pause(&self) {
        match self.mpv.pause() {
            Ok(_) => {}
            Err(e) => warn!("Could not pause: {}", e),
        }
    }

    pub fn unpause(&self) {
        match self.mpv.unpause() {
            Ok(_) => {}
            Err(e) => warn!("Could not unpause: {}", e),
        }
    }

    /// Stops and clears any song that is currently playing.
    pub fn stop(&self) {
        self.mpv.playlist_clear().ok();
    }

    /// Range 0..=100
    pub fn volume(&self) -> i64 {
        self.mpv.get_property(PROP_VOLUME).unwrap_or(0)
    }

    /// Range 0..=100
    pub fn set_volume(&self, volume: i64) {
        self.mpv
            .set_property(PROP_VOLUME, volume)
            .expect("Could not set volume");
    }

    ///  how much of the song has been played.
    pub fn current_song_seconds_played(&self) -> u64 {
        self.mpv.get_property(PROP_PLAYBACK_TIME).unwrap_or(0) as u64
    }

    pub fn seek_seconds_into_song(&self, seconds: u64) {
        // This will return an error when there is no song to be played. We can safely ignore it.
        self.mpv.seek_absolute(seconds as f64).ok();
        self.unpause();
    }

    pub fn current_song_length_in_seconds(&self) -> u64 {
        self.mpv.get_property(PROP_SONG_DURATION).unwrap_or(0) as u64
    }
}

impl Default for Playback {
    /// Will fail if the mpv library object cannot be created.
    fn default() -> Self {
        let mpv = Mpv::new().expect("Could not create mpv library instance.");

        // Don't show any video output.
        // Prevents mpv from showing an album or song image if it sees one.
        mpv.set_property(PROP_VIDEO_OUTPUT, "null").unwrap();
        // Pause when a song ends, instead of discarding the song.
        // This way we can detect the end of a song, and decide which to play next.
        mpv.set_property(PROP_PAUSE_WHEN_SONG_ENDS, "always")
            .unwrap();

        // TODO: Turn off the screensaver disabler? It can be done with the command line argument
        //    `--no_stop-screensaver`, but that doesn't take any data. So how to set it using `mpv.set_property`?

        let playback = Playback { mpv };

        playback.pause();
        // Start at less than 100 volume, so there is some leeway upwards.
        playback.set_volume(80);

        playback
    }
}
