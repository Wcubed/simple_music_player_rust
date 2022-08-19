use eframe::egui::{Context, DragValue, Grid, Widget, Window};
use rfd::FileDialog;
use simple_music_lib::config::Config;

#[derive(Default)]
pub struct ConfigView {
    window_open: bool,
}

impl ConfigView {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ctx: &Context, config: &mut Config) {
        Window::new("Config")
            .collapsible(false)
            .open(&mut self.window_open)
            .show(ctx, |ui| {
                Grid::new("config_grid")
                    .striped(true)
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Library:");
                        ui.label(format!("{}", config.library_directory.display()));

                        ui.end_row();

                        ui.label("");
                        if ui.button("Select library directory").clicked() {
                            if let Some(dir) = FileDialog::new().pick_folder() {
                                // TODO: let the user know when an error occured, with a pop-up or something like that.
                                config.library_directory = dir;
                            }
                        }
                        ui.end_row();

                        ui.label("Infinite playlist:");
                        ui.checkbox(&mut config.infinite_playlist, "");
                        ui.end_row();

                        ui.label("Infinite playlist buffer:");
                        DragValue::new(&mut config.infinite_playlist_song_buffer).ui(ui);
                        ui.end_row();

                        ui.label("Infinite playlist rear buffer:");
                        DragValue::new(&mut config.infinite_playlist_song_rear_buffer).ui(ui);
                        ui.end_row();
                    });
            });
    }

    pub fn open_window(&mut self) {
        self.window_open = true;
    }
}
