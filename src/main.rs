#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui::CtxRef;
use eframe::epi::Frame;
use eframe::{egui, epi};

struct App {
    items: Vec<bool>,
}

impl App {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            if ui.button("Add item").clicked() {
                self.items.push(false);
            }

            let mut remove_item = None;

            for (i, item) in self.items.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.checkbox(item, "Item");
                    if ui.button("X").clicked() {
                        remove_item = Some(i);
                    }
                });
            }

            if let Some(i) = remove_item {
                self.items.remove(i);
            }
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
