use crate::egui;
use crate::egui::Color32;
use eframe::egui::{CursorIcon, Grid, Id, RichText, Sense, Ui};
use simple_music_lib::library::{Library, ListEntryId, Playlist, SongId};

pub struct PlaylistView {
    dragged_item: Option<(ListEntryId, usize)>,
}

impl PlaylistView {
    pub fn new() -> Self {
        Self { dragged_item: None }
    }

    /// Returns `Some` when there is a request to play a song.
    pub fn show_playlist(
        &mut self,
        ui: &mut Ui,
        playlist: &mut Playlist,
        library: &Library,
        current_selected_entry: Option<(ListEntryId, SongId)>,
    ) -> Option<(ListEntryId, SongId)> {
        let mut remove_song_indexes = Vec::new();
        let mut request_play = None;

        if !ui.memory().is_anything_being_dragged() {
            self.dragged_item = None
        }

        if self.dragged_item.is_some() {
            ui.output().cursor_icon = CursorIcon::Grabbing;
        }

        let mut move_dragged_item_to_target_idx = None;

        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, playlist.song_count(), |ui, row_range| {
                Grid::new("playlist_grid")
                    .num_columns(3)
                    .start_row(row_range.start)
                    .min_col_width(1.0)
                    .striped(true)
                    .show(ui, |ui| {
                        for (idx, &(list_id, song_id)) in playlist
                            .song_ids()
                            .enumerate()
                            .skip(row_range.start)
                            .take(row_range.end)
                        {
                            if let Some(song) = library.get_song(&song_id) {
                                let id_source = "playlist_drag";
                                let ui_id = Id::new(id_source).with(list_id);

                                let rect = ui.label("::").rect;
                                let response = ui.interact(rect, ui_id, Sense::drag());

                                if response.drag_started() {
                                    self.dragged_item = Some((list_id, idx));
                                } else if response.hovered()
                                    && !ui.memory().is_anything_being_dragged()
                                {
                                    ui.output().cursor_icon = CursorIcon::Grab;
                                }

                                if let Some((dragged_id, _)) = self.dragged_item {
                                    if dragged_id != list_id {
                                        if let Some(last_pos) = ui.input().pointer.hover_pos() {
                                            if last_pos.y >= rect.top()
                                                && last_pos.y <= rect.bottom()
                                            {
                                                move_dragged_item_to_target_idx = Some(idx);
                                            }
                                        }
                                    }
                                }

                                if ui.button("x").clicked() {
                                    remove_song_indexes.push(idx);
                                }

                                let mut label_text = RichText::new(&song.title);
                                if let Some((dragged_id, _)) = self.dragged_item {
                                    if list_id == dragged_id {
                                        label_text = label_text
                                            .color(ui.style().interact(&response).text_color());
                                    }
                                }

                                if Some((list_id, song_id)) == current_selected_entry {
                                    label_text = label_text.color(Color32::LIGHT_BLUE);
                                }

                                let label = egui::Label::new(label_text).sense(Sense::click());
                                if ui.add(label).on_hover_text(&song.title).clicked() {
                                    request_play = Some((list_id, song_id));
                                }
                                ui.end_row();
                            }
                        }
                    });
            });

        if let (Some((item_id, from_index)), Some(target)) =
            (self.dragged_item, move_dragged_item_to_target_idx)
        {
            playlist.move_from_index_to_target_index(from_index, target);
            self.dragged_item = Some((item_id, target));
        }

        playlist.remove_songs_by_indexes(&remove_song_indexes);

        request_play
    }
}
