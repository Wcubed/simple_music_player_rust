#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod ui;

use crate::ui::library::LibraryView;
use crate::ui::playlist::{PlaylistAction, PlaylistView};
use crate::ui::time_label;
use anyhow::Result;
use eframe::egui::{Ui, Visuals};
use eframe::{egui, App, Storage};
use log::warn;
use log::LevelFilter;
use rfd::FileDialog;
use simple_music_lib::config::Config;
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
    config: Config,
    playback: Playback,
}

impl MusicApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let visuals = if let Some(dark_mode) = cc.integration_info.prefer_dark_mode {
            if dark_mode {
                Visuals::dark()
            } else {
                Visuals::light()
            }
        } else {
            Visuals::dark()
        };
        cc.egui_ctx.set_visuals(visuals);

        let mut app = Self {
            library: Library::new(),
            playlist: Playlist::new(),
            playlist_selected_song: None,
            playlist_view: PlaylistView::new(),
            library_view: LibraryView::new(),
            config,
            playback: Playback::new(),
        };

        app.scan_library_dir();

        app
    }

    /// Scans the library directory for songs.
    /// TODO: remove any songs that are no longer in the directory, add any that are new,
    ///       and update those that are already in the library.
    fn scan_library_dir(&mut self) {
        if self.config.library_directory.is_dir() {
            match library::scan_directory_for_songs(&self.config.library_directory) {
                Ok(songs) => {
                    self.library.add_songs(songs);
                    self.library_view.update_items(&self.library);
                }
                Err(e) => warn!("Something went wrong while scanning for songs: '{}'", e),
            }
        }
    }

    fn show_library(&mut self, ui: &mut Ui) {
        if ui.button("Select library directory").clicked() {
            if let Some(dir) = FileDialog::new().pick_folder() {
                // TODO: let the user know when error occured, with a pop-up or something like that.
                self.config.library_directory = dir;
                self.scan_library_dir();
            }
        }

        let add_songs = self.library_view.show_library(ui);

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
        }
    }

    fn play_previous_song(&mut self) {
        let next_entry = if let Some(cur_entry) = self.playlist_selected_song {
            self.playlist.get_previous_entry(cur_entry.0)
        } else {
            self.playlist.get_last_entry()
        };

        if let Some(entry) = next_entry {
            self.play_playlist_entry(entry);
        }
    }

    fn play_playlist_entry(&mut self, entry: (ListEntryId, SongId)) {
        if let Some(song) = self.library.get_song(&entry.1) {
            match self.playback.play_file(&song.path) {
                Ok(()) => self.playlist_selected_song = Some(entry),
                Err(e) => warn!("Could not play song `{}`: {}", song.path.display(), e),
            }
        }
        self.playback.unpause();
    }

    fn show_playback_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("|<").clicked() {
                self.play_previous_song();
            }

            let paused = self.playback.is_paused();

            if paused {
                if ui.button(">").clicked() {
                    if self.playlist_selected_song.is_some() {
                        self.playback.unpause();
                    } else {
                        self.play_next_song();
                    }
                }
            } else if ui.button("||").clicked() {
                self.playback.pause();
            }

            if ui.button(">|").clicked() {
                self.play_next_song();
            }

            let mut volume = self.playback.volume();
            ui.add(egui::Slider::new(&mut volume, 0..=100).show_value(false));
            self.playback.set_volume(volume);

            let seconds_played = self.playback.current_song_seconds_played();
            let total_length = self.playback.current_song_length_in_seconds();

            time_label(ui, seconds_played);
            time_label(ui, total_length);

            let fraction_played = seconds_played as f32 / total_length as f32;

            ui.add(egui::ProgressBar::new(fraction_played));

            if !paused {
                // TODO: repaint every second.
            }
        });
    }
}

impl App for MusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.show_playback_controls(ui);
        });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .min_width(10.0)
            .show(ctx, |ui| {
                // TODO: Add a way of selecting a library.
                self.show_library(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            let action = self.playlist_view.show(
                ui,
                &mut self.playlist,
                &self.library,
                self.playlist_selected_song,
            );

            match action {
                PlaylistAction::PlaySong(entry) => {
                    self.play_playlist_entry(entry);
                }
                PlaylistAction::RemoveSong(remove_id) => {
                    if let Some((selected_id, _)) = self.playlist_selected_song {
                        if selected_id == remove_id {
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

fn main() -> Result<()> {
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
