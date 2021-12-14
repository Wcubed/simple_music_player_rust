use anyhow::Result;
use simple_music_lib::library;
use simple_music_lib::library::Song;
use std::path::PathBuf;

const TEST_LIBRARY_DIRECTORY: &str = "test_assets/test_library";

#[test]
fn test_scan_directory() -> Result<()> {
    let files = library::scan_directory_for_songs(TEST_LIBRARY_DIRECTORY)?;

    assert_eq!(files.len(), 2);

    let noise_song = Song {
        title: "noise".to_string(),
        path: PathBuf::from(TEST_LIBRARY_DIRECTORY).join("noise.ogg"),
    };
    assert!(files.contains(&noise_song));

    let more_noise_song = Song {
        title: "more_noise".to_string(),
        path: PathBuf::from(TEST_LIBRARY_DIRECTORY)
            .join("some_folder")
            .join("more_noise.ogg"),
    };
    assert!(files.contains(&more_noise_song));

    Ok(())
}
