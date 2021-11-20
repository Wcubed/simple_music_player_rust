#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod library;

use crate::library::{Library, Playlist, Song};
use eframe::egui::{CtxRef, Ui};
use eframe::epi::{Frame, Storage};
use eframe::{egui, epi};

struct App {
    library: Library,
    playlist: Playlist,
}

impl App {
    fn new() -> Self {
        Self {
            library: Library::new(),
            playlist: Playlist::new(),
        }
    }

    fn show_library(&mut self, ui: &mut Ui) {
        let mut add_songs = Vec::new();

        egui::Grid::new("library_grid")
            .num_columns(2)
            .min_col_width(1.0)
            .striped(true)
            .show(ui, |ui| {
                for (&id, song) in self.library.songs() {
                    if self.show_library_song(ui, song) {
                        add_songs.push(id);
                    }
                }
            });

        self.playlist.add_songs(add_songs);
    }

    fn show_library_song(&self, ui: &mut Ui, song: &Song) -> bool {
        let mut add_song = false;
        add_song = ui.button("+").clicked();
        ui.label(&song.title);

        ui.end_row();
        add_song
    }

    fn show_playlist(&mut self, ui: &mut Ui) {
        let mut remove_song_indexes = Vec::new();

        egui::Grid::new("playlist_grid")
            .num_columns(2)
            .min_col_width(1.0)
            .striped(true)
            .show(ui, |ui| {
                for (idx, id) in self.playlist.song_ids().enumerate() {
                    if let Some(song) = self.library.get_song(id) {
                        ui.label(&song.title);
                        if ui.button("x").clicked() {
                            remove_song_indexes.push(idx);
                        }
                        ui.end_row();
                    }
                }
            });

        self.playlist.remove_songs_by_indexes(&remove_song_indexes);
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::SidePanel::right("library_panel").show(ctx, |ui| {
            self.show_library(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_playlist(ui);
        });
    }

    fn setup(&mut self, _ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        // Start with some songs for testing purposes.
        self.library.add_song(Song {
            title: "Blaaargh!!!".into(),
        });
        self.library.add_song(Song {
            title: "Super epic metal song!".into(),
        });
        self.library.add_song(Song {
            title: "Orchestral cover song".into(),
        })
    }

    fn name(&self) -> &str {
        "Example egui App"
    }
}

fn main() {
    let app = App::new();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
