use eframe::egui;
use eframe::egui::Ui;
use simple_music_lib::library::{Library, Song, SongId};

pub struct LibraryView {
    shown_items: Vec<(SongId, Song)>,
}

impl LibraryView {
    pub fn new() -> Self {
        Self {
            shown_items: vec![],
        }
    }

    pub fn update_items(&mut self, library: &Library) {
        self.shown_items.clear();

        //  TODO (Wybe 2022-04-29): Copying the songs every time is not the most efficient way.
        //                          but pre-mature optimization is also a thing.
        for (&id, song) in library.songs() {
            self.shown_items.push((id, song.clone()));
        }

        self.shown_items
            .sort_by(|(_, first), (_, second)| first.title.partial_cmp(&second.title).unwrap());
    }

    /// Returns a list of songs to add to the playlist.
    pub fn show_library(&mut self, ui: &mut Ui) -> Vec<SongId> {
        let mut add_songs = Vec::new();

        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, self.shown_items.len(), |ui, row_range| {
                egui::Grid::new("library_grid")
                    .num_columns(2)
                    .start_row(row_range.start)
                    .min_col_width(1.0)
                    .striped(true)
                    .show(ui, |ui| {
                        for (id, song) in self
                            .shown_items
                            .iter()
                            .skip(row_range.start)
                            .take(row_range.end)
                        {
                            if self.show_song(ui, song) {
                                add_songs.push(*id);
                            }
                        }
                    });
            });

        add_songs
    }

    fn show_song(&self, ui: &mut Ui, song: &Song) -> bool {
        let add_song = ui.button("+").clicked();
        ui.label(&song.title).on_hover_text(&song.title);

        ui.end_row();
        add_song
    }
}
