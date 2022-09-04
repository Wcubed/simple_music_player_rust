#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod ui;

use crate::egui::Sense;
use crate::ui::config_ui::ConfigView;
use crate::ui::library::LibraryView;
use crate::ui::playback_controls::{PlaybackCommand, PlaybackControls};
use crate::ui::playlist::{PlaylistAction, PlaylistView};
use crate::ui::time_label;
use anyhow::Result;
use eframe::egui::{Ui, Visuals, Widget};
use eframe::{egui, App, Storage};
use log::LevelFilter;
use log::{info, warn};
use simple_music_lib::config::Config;
use simple_music_lib::image_cache::ImageCache;
use simple_music_lib::library;
use simple_music_lib::library::{Library, ListEntryId, Playlist, SongId};
use simple_music_lib::playback::Playback;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::time::Duration;

struct MusicApp {
    library: Library,
    playlist: Playlist,
    playlist_selected_song: Option<(ListEntryId, SongId)>,
    playlist_view: PlaylistView,
    library_view: LibraryView,
    playback_controls: PlaybackControls,
    config: Config,
    config_view: ConfigView,
    playback: Playback,
    image_cache: ImageCache,
}

impl MusicApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let visuals = Visuals::dark();
        cc.egui_ctx.set_visuals(visuals);

        let mut app = Self {
            library: Library::new(),
            playlist: Playlist::new(),
            playlist_selected_song: None,
            playlist_view: PlaylistView::new(),
            library_view: LibraryView::new(),
            playback_controls: PlaybackControls::new(),
            config,
            config_view: ConfigView::new(),
            playback: Playback::new(),
            image_cache: ImageCache::new(),
        };

        app.scan_library_dir(&cc.egui_ctx);

        app
    }

    /// Scans the library directory for songs.
    /// TODO: remove any songs that are no longer in the directory, add any that are new,
    ///       and update those that are already in the library.
    fn scan_library_dir(&mut self, ctx: &egui::Context) {
        if self.config.library_directory.is_dir() {
            self.library.clear();
            self.playlist.clear();

            match library::scan_directory_for_songs(&self.config.library_directory) {
                Ok(songs) => {
                    self.library.add_songs(songs);
                    self.library_view.update_items(&self.library);

                    for (&id, song) in self.library.songs() {
                        self.image_cache
                            .load_image_from_song_path(ctx, &song.path, id);
                    }
                }
                Err(e) => warn!("Something went wrong while scanning for songs: '{}'", e),
            }
        }
    }

    fn show_library(&mut self, ui: &mut Ui) {
        let add_songs = self.library_view.show_library(ui, &self.image_cache);

        self.playlist.add_songs(add_songs);
    }

    fn play_next_song(&mut self) {
        let next_entry = if let Some(cur_entry) = self.playlist_selected_song {
            self.playlist.get_next_entry(cur_entry.0)
        } else {
            self.playlist.get_first_entry()
        };

        if let Some(entry) = next_entry {
            self.play_playlist_entry(entry);
        } else {
            // No more songs in the list.
            if self.config.infinite_playlist {
                // TODO: Remember which songs have already played, and don't select those?
                if let Some(&random_song_id) = self.library.get_random_song_id() {
                    self.playlist.add_song(random_song_id);
                    // This recursive call should be fine, because we just added another song to play.
                    // But it would likely be better if there wasn't recursion here.
                    self.play_next_song();
                } else {
                    self.stop_playing();
                }
            } else {
                self.stop_playing();
            }
        }
    }

    /// Stops playing, and sets the current song to `None`.
    fn stop_playing(&mut self) {
        self.playlist_selected_song = None;
        self.playback.stop();
    }

    fn play_previous_song(&mut self) {
        let prev_entry = if let Some(cur_entry) = self.playlist_selected_song {
            self.playlist.get_previous_entry(cur_entry.0)
        } else {
            self.playlist.get_last_entry()
        };

        if let Some(entry) = prev_entry {
            self.play_playlist_entry(entry);
        }
    }

    fn play_playlist_entry(
        &mut self,
        (entry_id, song_id, entry_index): (ListEntryId, SongId, usize),
    ) {
        if let Some(song) = self.library.get_song(&song_id) {
            match self.playback.play_file(&song.path) {
                Ok(()) => self.playlist_selected_song = Some((entry_id, song_id)),
                Err(e) => warn!("Could not play song `{}`: {}", song.path.display(), e),
            }
        }
        self.playback.unpause();

        if self.config.infinite_playlist {
            // Fill the playlist with random songs until we have the desired amount of buffer.
            let songs_in_buffer = self.playlist.length() - (entry_index + 1);
            let desired_buffer = self.config.infinite_playlist_song_buffer as usize;

            if songs_in_buffer < desired_buffer {
                for _ in songs_in_buffer..desired_buffer {
                    if let Some(&random_song) = self.library.get_random_song_id() {
                        self.playlist.add_song(random_song);
                    }
                }
            }

            // Remove songs from the back until we are left with the desired amount of rear buffer.
            let songs_in_rear_buffer = entry_index;
            let desired_rear_buffer = self.config.infinite_playlist_song_rear_buffer as usize;
            if songs_in_rear_buffer > desired_rear_buffer {
                for _ in desired_rear_buffer..songs_in_rear_buffer {
                    self.playlist.remove_song_by_index(0);
                }
            }
        }
    }

    fn show_playback_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let paused = self.playback.is_paused();
            let volume = self.playback.volume();

            if let Some(command) =
                self.playback_controls
                    .show(ui, paused, volume, &mut self.config.infinite_playlist)
            {
                match command {
                    PlaybackCommand::Pause => self.playback.pause(),
                    PlaybackCommand::Unpause => {
                        if self.playlist_selected_song.is_some() {
                            self.playback.unpause();
                        } else {
                            self.play_next_song();
                        }
                    }
                    PlaybackCommand::NextSong => self.play_next_song(),
                    PlaybackCommand::PreviousSong => self.play_previous_song(),
                    PlaybackCommand::SetVolume(new_volume) => self.playback.set_volume(new_volume),
                }
            }

            let seconds_played = self.playback.current_song_seconds_played();
            let total_length = self.playback.current_song_length_in_seconds();

            time_label(ui, seconds_played);
            ui.label("/");
            time_label(ui, total_length);

            let fraction_played = seconds_played as f32 / total_length as f32;

            let response = egui::ProgressBar::new(fraction_played).ui(ui);
            // Progress bar doesn't listen for clicks by default, so we do it after it is drawn.
            let response = response.interact(Sense::click_and_drag());

            if let Some(interact_pos) = response.interact_pointer_pos() {
                if response.drag_released() || response.clicked() {
                    let x_on_bar = interact_pos.x - response.rect.min.x;
                    let bar_width = response.rect.width();
                    let fraction = x_on_bar / bar_width;

                    let seconds_selected = (total_length as f32 * fraction).floor() as u64;

                    if self.playlist_selected_song.is_some() {
                        self.playback.seek_seconds_into_song(seconds_selected);
                    }
                }
            }

            if seconds_played == total_length && seconds_played != 0 {
                // Song has ended. Play next song.
                self.play_next_song();
            }

            if !paused {
                // If we are playing music, we need to update the UI periodically,
                // otherwise the song progress will not be shown.
                // And we would not realize that a song has finished playing.
                ui.ctx().request_repaint_after(Duration::from_secs(1));
            }
        });
    }
}

impl App for MusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let prev_library_directory = self.config.library_directory.clone();

        self.config_view.show(ctx, &mut self.config);

        if prev_library_directory != self.config.library_directory {
            self.scan_library_dir(ctx);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Config").clicked() {
                    self.config_view.open_window();
                }

                let add_songs = self.library_view.show_library_search_widget(ui);
                self.playlist.add_songs(add_songs);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.show_playback_controls(ui);
        });

        if self.library_view.should_show_library() {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .min_width(10.0)
                .show(ctx, |ui| {
                    self.show_library(ui);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let action = self.playlist_view.show(
                ui,
                &mut self.playlist,
                &self.library,
                &self.image_cache,
                self.playlist_selected_song,
            );

            match action {
                PlaylistAction::PlaySong((list_entry, song_id)) => {
                    if let Some(index) = self.playlist.get_song_index(list_entry) {
                        self.play_playlist_entry((list_entry, song_id, index));
                    }
                }
                PlaylistAction::RemoveSong(remove_id) => {
                    if let Some((selected_id, _)) = self.playlist_selected_song {
                        if selected_id == remove_id {
                            // TODO: Make it not play a song if you remove the last song from the playlist
                            //   (because the "next song" will be the current song, because it hasn't been removed yet).
                            self.play_next_song();
                        }
                    }
                    self.playlist.remove_song(remove_id)
                }
                PlaylistAction::None => {}
            }
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        ConfigBuilder::default()
            .set_thread_level(LevelFilter::Trace)
            .set_target_level(LevelFilter::Trace)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Simple music player",
        native_options,
        Box::new(|cc| Box::new(MusicApp::new(cc))),
    );
}
