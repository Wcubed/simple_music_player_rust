use crate::egui;
use crate::egui::Color32;
use egui::{CursorIcon, Grid, Id, RichText, Sense, Ui};
use simple_music_lib::image_cache::ImageCache;
use simple_music_lib::library::{Library, ListEntryId, Playlist, SongId};

pub enum PlaylistAction {
    None,
    PlaySong((ListEntryId, SongId)),
    RemoveSong(ListEntryId),
}

pub struct PlaylistView {
    dragged_item: Option<(ListEntryId, usize)>,
}

impl PlaylistView {
    pub fn new() -> Self {
        Self { dragged_item: None }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        playlist: &mut Playlist,
        library: &Library,
        image_cache: &ImageCache,
        current_selected_entry: Option<(ListEntryId, SongId)>,
    ) -> PlaylistAction {
        let mut action = PlaylistAction::None;

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
                    .num_columns(4)
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
                                    action = PlaylistAction::RemoveSong(list_id);
                                }

                                // TODO: allow caching and retrieving pre-scaled versions of images.
                                if let Some(texture_handle) =
                                    image_cache.get_texture_handle(song_id)
                                {
                                    // TODO: get image width scaled relative to it's normal height, and not the target height.
                                    ui.image(texture_handle, [row_height / 9.0 * 16.0, row_height]);
                                } else {
                                    ui.label("");
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
                                if ui
                                    .add(label)
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .on_hover_text(&song.title)
                                    .clicked()
                                    && Some((list_id, song_id)) != current_selected_entry
                                {
                                    action = PlaylistAction::PlaySong((list_id, song_id));
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

        action
    }
}
