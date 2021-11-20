#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod library;

use crate::library::{Library, Song};
use eframe::egui::CtxRef;
use eframe::epi::{Frame, Storage};
use eframe::{egui, epi};

struct App {
    library: Library,
}

impl App {
    fn new() -> Self {
        Self {
            library: Library::new(),
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (_id, song) in self.library.songs() {
                ui.horizontal(|ui| {
                    ui.label(&song.title);
                });
            }
        });
    }

    fn setup(&mut self, _ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        // Start with some songs for testing purposes.
        self.library.add_song(Song {
            title: "Blaaargh!!!".into(),
        });
        self.library.add_song(Song {
            title: "Super epic metal song!".into(),
        });
        self.library.add_song(Song {
            title: "Orchestral cover song".into(),
        })
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
