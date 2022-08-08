use eframe::egui::Ui;

pub mod library;
pub mod playlist;

pub fn time_label(ui: &mut Ui, seconds: u64) {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    // TODO: make this time label a separate function.
    ui.label(format!("{}:{:02}:{:02}", hours, minutes, seconds));
}
