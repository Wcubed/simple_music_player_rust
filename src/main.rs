#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod library;

use crate::library::{Library, ListEntryId, Playlist, Song};
use eframe::egui::{CtxRef, CursorIcon, Id, Sense, Ui};
use eframe::epi::{Frame, Storage};
use eframe::{egui, epi};

struct App {
    library: Library,
    playlist: Playlist,
    playlist_view: PlaylistView,
}

impl App {
    fn new() -> Self {
        Self {
            library: Library::new(),
            playlist: Playlist::new(),
            playlist_view: PlaylistView::new(),
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
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::SidePanel::right("library_panel").show(ctx, |ui| {
            self.show_library(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.playlist_view
                .show_playlist(ui, &mut self.playlist, &self.library)
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

struct PlaylistView {
    dragged_item: Option<(ListEntryId, usize)>,
}

impl PlaylistView {
    pub fn new() -> Self {
        Self { dragged_item: None }
    }

    pub fn show_playlist(&mut self, ui: &mut Ui, playlist: &mut Playlist, library: &Library) {
        let mut remove_song_indexes = Vec::new();

        if !ui.memory().is_anything_being_dragged() {
            self.dragged_item = None
        }

        if self.dragged_item.is_some() {
            ui.output().cursor_icon = CursorIcon::Grabbing;
        }

        let mut move_dragged_item_to_target_idx = None;

        egui::Grid::new("playlist_grid")
            .num_columns(3)
            .min_col_width(1.0)
            .striped(true)
            .show(ui, |ui| {
                for (idx, &(list_id, song_id)) in playlist.song_ids().enumerate() {
                    if let Some(song) = library.get_song(&song_id) {
                        let id_source = "playlist_drag";
                        let ui_id = Id::new(id_source).with(list_id);

                        let rect = ui.label("::").rect;
                        let response = ui.interact(rect, ui_id, Sense::drag());

                        if response.drag_started() {
                            self.dragged_item = Some((list_id, idx));
                        } else if response.hovered() && !ui.memory().is_anything_being_dragged() {
                            ui.output().cursor_icon = CursorIcon::Grab;
                        }

                        if let Some((dragged_id, _)) = self.dragged_item {
                            if dragged_id != list_id {
                                if let Some(last_pos) = ui.input().pointer.hover_pos() {
                                    if last_pos.y >= rect.top() && last_pos.y <= rect.bottom() {
                                        move_dragged_item_to_target_idx = Some(idx);
                                    }
                                }
                            }
                        }

                        ui.label(&song.title);
                        if ui.button("x").clicked() {
                            remove_song_indexes.push(idx);
                        }
                        ui.end_row();
                    }
                }
            });

        if let (Some((item_id, from_index)), Some(target)) =
            (self.dragged_item, move_dragged_item_to_target_idx)
        {
            playlist.move_from_index_to_target_index(from_index, target);
            self.dragged_item = Some((item_id, target));
        }

        playlist.remove_songs_by_indexes(&remove_song_indexes);
    }
}

fn main() {
    let app = App::new();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
