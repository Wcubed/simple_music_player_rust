use eframe::egui::Context;
use simple_music_lib::image_cache::ImageCache;
use simple_music_lib::library::{Library, Song, SongId};
use std::path::PathBuf;
use test_log::test;

/// TODO: This is also used in other integration test files. How do we make it commonly available between them.
const TEST_LIBRARY_DIRECTORY: &str = "test_assets/test_library";

#[test]
fn test_load_image_from_song() {
    // Arrange
    let mut cache = ImageCache::new();

    // Use the library to obtain a song id.
    let mut library = Library::new();
    let id = library.add_song(Song {
        title: "".to_string(),
        path: Default::default(),
    });

    let ctx = Context::default();

    // Act
    cache.load_image_from_song_path(
        ctx,
        &PathBuf::from(TEST_LIBRARY_DIRECTORY).join("noise.ogg"),
        id,
    );

    // Assert
    assert!(cache.get_texture_handle(id).is_some());
}
