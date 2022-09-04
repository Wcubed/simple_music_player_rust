use crate::library::SongId;
use eframe::egui::{ColorImage, Context, TextureFilter, TextureHandle};
use log::{debug, warn};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct ImageCache {
    images: HashMap<SongId, TextureHandle>,
}

impl ImageCache {
    pub fn new() -> ImageCache {
        Default::default()
    }

    /// Searches for an image in the same folder as the song, and with the same name.
    /// Loads it if it finds it, and adds it to the cache.
    /// The handle can later be retrieved with the [SongId] using [get_texture_handle](ImageCache::get_texture_handle)
    /// TODO: Load in the background.
    /// TODO: Allow specifying resizing the image, and caching those smaller images on disk.
    /// TODO: If there is already an image in the cache for this song, how do we make egui drop the texture for it?
    pub fn load_image_from_song_path(&mut self, ctx: Context, song_path: &Path, song_id: SongId) {
        let image_extensions = ["jpg", "png", "webp"];
        let file_name = match song_path.file_stem() {
            Some(value) => value,
            None => return, // Can't load an image if we don't have a name to look for. TODO: add logging.
        };
        let directory = match song_path.parent() {
            Some(value) => value,
            None => return, // Can't load an image if we don't have a directory to look in. TODO: add logging.
        };

        let image_base_path = PathBuf::from(directory).join(file_name);

        let image_data = {
            let mut data = None;

            for extension in image_extensions {
                let full_path = image_base_path.with_extension(extension);

                if !full_path.exists() {
                    // No image with this extensions, try the next extension.
                    continue;
                }

                match image::open(&full_path) {
                    Ok(img) => {
                        let img = img.to_rgba8();
                        let pixels = img.as_flat_samples();
                        let size = [img.width() as _, img.height() as _];

                        data = Some(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()));

                        // Found an image with this extension,
                        // no need to look for the other extensions.
                        break;
                    }
                    Err(e) => {
                        warn!("Cannot open image `{}`: {}", full_path.display(), e)
                    }
                }
            }

            data
        };

        if let Some(data) = image_data {
            self.images.insert(
                song_id,
                ctx.load_texture(file_name.to_string_lossy(), data, TextureFilter::Linear),
            );
        } else {
            debug!(
                "No image found for song `{}` in directory `{}`",
                file_name.to_string_lossy(),
                directory.display()
            );
        }
    }

    pub fn get_texture_handle(&self, song_id: SongId) -> Option<&TextureHandle> {
        self.images.get(&song_id)
    }
}
