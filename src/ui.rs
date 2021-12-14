use crate::library::ListEntryId;
use crate::{Id, Library, Playlist};
use eframe::egui;
use eframe::egui::{CursorIcon, Sense, Ui, Widget};

pub struct PlaylistView {
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

                        let mut label = egui::Label::new(&song.title);
                        if let Some((dragged_id, _)) = self.dragged_item {
                            if list_id == dragged_id {
                                label =
                                    label.text_color(ui.style().interact(&response).text_color());
                            }
                        }

                        label.ui(ui);
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
