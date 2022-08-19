use eframe::egui;
use eframe::egui::{Key, Modifiers, Ui};
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

        let search_bar = egui::TextEdit::singleline(&mut self.filter_string)
            .hint_text("Search")
            .desired_width(200.0);
        let search_bar_response = ui.add(search_bar);

        let search_bar_enter_pressed =
            search_bar_response.lost_focus() && ui.input().key_pressed(Key::Enter);

        // Ctrl-F focuses on the search bar.
        // TODO: Have an app-wide way of detecting shortcuts?
        if ui.input().key_pressed(Key::F) && ui.input().modifiers.matches(Modifiers::COMMAND) {
            search_bar_response.request_focus();
        }

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

        if search_bar_enter_pressed {
            if let Some(&(first_id, _)) = self.filtered_items.first() {
                add_songs.push(first_id);

                // Clear the search bar
                self.update_filter_string(String::new());
            }

            // Because by default egui loses focus on a TextEdit when the enter key is pressed
            // we need to re-focus on it.
            // Especially if no song was found, because then we haven't
            // actually done anything yet.
            search_bar_response.request_focus();
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

    /// Whether or not the library wants to be displayed or not.
    pub fn should_show_library(&self) -> bool {
        !self.filter_string.is_empty()
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
