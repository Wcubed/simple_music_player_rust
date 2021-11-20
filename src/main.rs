#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui::CtxRef;
use eframe::epi::Frame;
use eframe::{egui, epi};

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
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
