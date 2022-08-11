use eframe::egui;
use eframe::egui::{Key, Ui};
use simple_music_lib::library::{Library, Song, SongId};

pub struct LibraryView {
    filter_string: String,
    /// Always sorted alphabetically.
    unfiltered_items: Vec<(SongId, Song)>,
    filtered_items: Vec<(SongId, Song)>,
}

impl LibraryView {
    pub fn new() -> Self {
        Self {
            filter_string: String::new(),
            unfiltered_items: vec![],
            filtered_items: vec![],
        }
    }

    pub fn update_items(&mut self, library: &Library) {
        self.unfiltered_items.clear();

        //  TODO (Wybe 2022-04-29): Copying the songs every time is not the most efficient way.
        //                          but pre-mature optimization is also a thing.
        for (&id, song) in library.songs() {
            self.unfiltered_items.push((id, song.clone()));
        }

        self.unfiltered_items
            .sort_by(|(_, first), (_, second)| first.title.partial_cmp(&second.title).unwrap());

        self.filtered_items = self.unfiltered_items.clone();
    }

    /// Returns a list of songs to add to the playlist.
    pub fn show_library_search_widget(&mut self, ui: &mut Ui) -> Vec<SongId> {
        let prev_filter_string = self.filter_string.clone();

        // TODO: somehow keep or return focus to the edit after the "enter" key is pressed.
        let filter_edit = egui::TextEdit::singleline(&mut self.filter_string)
            .hint_text("Search")
            .desired_width(200.0);

        let filter_edit_enter_pressed =
            ui.add(filter_edit).lost_focus() && ui.input().key_pressed(Key::Enter);

        if prev_filter_string != self.filter_string {
            self.update_filter_string(self.filter_string.clone());
        }

        if self.filtered_items.len() == self.unfiltered_items.len() {
            ui.label(format!("{} songs", self.unfiltered_items.len()));
        } else {
            ui.label(format!(
                "{} / {} songs",
                self.filtered_items.len(),
                self.unfiltered_items.len()
            ));
        }

        let mut add_songs = Vec::new();

        if filter_edit_enter_pressed {
            if let Some(&(first_id, _)) = self.filtered_items.first() {
                add_songs.push(first_id);
            }

            self.update_filter_string(String::new());
        }

        add_songs
    }

    fn update_filter_string(&mut self, new_string: String) {
        self.filter_string = new_string;

        let lowercase_filter = self.filter_string.to_lowercase();
        self.filtered_items = self
            .unfiltered_items
            .iter()
            .filter(|(_, song)| song.title.to_lowercase().contains(&lowercase_filter))
            .cloned()
            .collect();
    }

    /// Returns a list of songs to add to the playlist.
    pub fn show_library(&mut self, ui: &mut Ui) -> Vec<SongId> {
        let mut add_songs = Vec::new();

        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show_rows(
                ui,
                row_height,
                self.filtered_items.len(),
                |ui, row_range| {
                    egui::Grid::new("library_grid")
                        .num_columns(2)
                        .start_row(row_range.start)
                        .min_col_width(1.0)
                        .striped(true)
                        .show(ui, |ui| {
                            for (id, song) in self
                                .filtered_items
                                .iter()
                                .skip(row_range.start)
                                .take(row_range.end)
                            {
                                if self.show_song(ui, song) {
                                    add_songs.push(*id);
                                }
                            }
                        });
                },
            );

        add_songs
    }

    fn show_song(&self, ui: &mut Ui, song: &Song) -> bool {
        let add_song = ui.button("+").clicked();
        ui.label(&song.title).on_hover_text(&song.title);

        ui.end_row();
        add_song
    }
}
