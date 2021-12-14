use anyhow::Result;
use simple_music_lib::library;

const TEST_LIBRARY_DIRECTORY: &str = "test_assets/test_library";

#[test]
fn test_scan_directory() -> Result<()> {
    let files = library::scan_directory(TEST_LIBRARY_DIRECTORY);

    Ok(())
}
