#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod ui;

use crate::ui::PlaylistView;
use anyhow::{Context, Result};
use eframe::egui::Ui;
use eframe::epi::{Frame, Storage};
use eframe::{egui, epi};
use log::warn;
use log::LevelFilter;
use rodio::{OutputStream, OutputStreamHandle};
use simple_music_lib::config::Config;
use simple_music_lib::library;
use simple_music_lib::library::{Library, Playlist, Song};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::fs::File;
use std::io::{Read, Write};

const SAVE_FILE_NAME: &str = "config.toml";

struct App {
    library: Library,
    playlist: Playlist,
    playlist_view: PlaylistView,
    slider_value: f32,
    config: Config,
    _output_stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl App {
    fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;

        Ok(Self {
            library: Library::new(),
            playlist: Playlist::new(),
            playlist_view: PlaylistView::new(),
            slider_value: 0.0,
            config: Config::default(),
            _output_stream: stream,
            stream_handle,
        })
    }

    fn save_config(&self) -> Result<()> {
        let serialized = toml::to_string(&self.config)?;
        let mut file = File::create(SAVE_FILE_NAME).context("Could not create config file")?;
        file.write_all(serialized.as_bytes())
            .context("Could not serialize config to toml format")?;

        Ok(())
    }

    fn load_config(&mut self) -> Result<()> {
        let mut file = File::open(SAVE_FILE_NAME).context("Could not open config file")?;
        let mut serialized = String::new();
        file.read_to_string(&mut serialized)
            .context("Could not parse config toml")?;

        self.config = toml::from_str(&serialized)?;

        Ok(())
    }

    fn show_library(&mut self, ui: &mut Ui) {
        let mut add_songs = Vec::new();

        egui::Grid::new("library_grid")
            .num_columns(2)
            .min_col_width(1.0)
            .striped(true)
            .show(ui, |ui| {
                for (&id, song) in self.library.songs() {
                    if self.show_library_song(ui, song) {
                        add_songs.push(id);
                    }
                }
            });

        self.playlist.add_songs(add_songs);
    }

    fn show_library_song(&self, ui: &mut Ui, song: &Song) -> bool {
        let add_song = ui.button("+").clicked();
        ui.label(&song.title);

        ui.end_row();
        add_song
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &Frame) {
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            self.show_library(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.playlist_view
                .show_playlist(ui, &mut self.playlist, &self.library)
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Text");
                ui.add(egui::ProgressBar::new(self.slider_value));
            });
        });
    }

    fn setup(&mut self, _ctx: &egui::Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        if let Err(e) = self.load_config() {
            warn!("Encountered a problem while loading config: {}", e);
        }

        if self.config.library_directory.is_dir() {
            match library::scan_directory_for_songs(&self.config.library_directory) {
                Ok(songs) => {
                    self.library.add_songs(songs);
                }
                Err(e) => warn!("Something went wrong while scanning for songs: '{}'", e),
            }
        }

        if let Some((_, song)) = self.library.songs().next() {
            library::play_song_from_file(&song.path, &self.stream_handle);
        }
    }

    fn on_exit(&mut self) {
        if let Err(e) = self.save_config() {
            warn!("Encountered a problem while saving config: {}", e);
        }
    }

    fn name(&self) -> &str {
        "Example egui App"
    }
}

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Warn,
        ConfigBuilder::default()
            .set_thread_level(LevelFilter::Trace)
            .set_target_level(LevelFilter::Trace)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let app = App::new();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app?), native_options);
}