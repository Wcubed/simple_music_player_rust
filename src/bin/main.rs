#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod ui;

use crate::egui::Label;
use crate::ui::PlaylistView;
use anyhow::{Context, Result};
use eframe::egui::Ui;
use eframe::epi::{Frame, Storage};
use eframe::{egui, epi};
use log::warn;
use log::LevelFilter;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use simple_music_lib::config::Config;
use simple_music_lib::library;
use simple_music_lib::library::{Library, ListEntryId, Playlist, Song, SongId};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::fs::File;
use std::io::{Read, Write};

const SAVE_FILE_NAME: &str = "config.toml";

struct App {
    library: Library,
    playlist: Playlist,
    playlist_selected_song: Option<(ListEntryId, SongId)>,
    playlist_view: PlaylistView,
    slider_value: f32,
    config: Config,
    _output_stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    audio_sink: Sink,
}

impl App {
    fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.pause();

        Ok(Self {
            library: Library::new(),
            playlist: Playlist::new(),
            playlist_selected_song: None,
            playlist_view: PlaylistView::new(),
            slider_value: 0.0,
            config: Config::default(),
            _output_stream: stream,
            _stream_handle: stream_handle,
            audio_sink: sink,
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

        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show_rows(
                ui,
                row_height,
                self.library.song_count(),
                |ui, row_range| {
                    egui::Grid::new("library_grid")
                        .num_columns(2)
                        .start_row(row_range.start)
                        .min_col_width(1.0)
                        .striped(true)
                        .show(ui, |ui| {
                            for (&id, song) in self
                                .library
                                .songs()
                                .skip(row_range.start)
                                .take(row_range.end)
                            {
                                if self.show_library_song(ui, song) {
                                    add_songs.push(id);
                                }
                            }
                        });
                },
            );

        self.playlist.add_songs(add_songs);
    }

    fn show_library_song(&self, ui: &mut Ui, song: &Song) -> bool {
        let add_song = ui.button("+").clicked();
        ui.label(&song.title).on_hover_text(&song.title);

        ui.end_row();
        add_song
    }

    /// Returns None if no song could be played.
    /// Returns Some(song) if a song is now playing.
    fn play_next_song(&mut self) {
        let next_entry = if let Some(cur_entry) = self.playlist_selected_song {
            self.playlist.get_next_entry(cur_entry.0)
        } else {
            self.playlist.get_first_entry()
        };

        if let Some(entry) = next_entry {
            self.play_playlist_entry(entry);
        }
    }

    fn play_playlist_entry(&mut self, entry: (ListEntryId, SongId)) {
        if let Some(song) = self.library.get_song(&entry.1) {
            match library::play_song_from_file(&song.path, &self.audio_sink) {
                Ok(()) => self.playlist_selected_song = Some(entry),
                Err(e) => warn!("Could not play song `{}`: {}", song.path.display(), e),
            }
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &Frame) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.audio_sink.is_paused() {
                    if ui.button(">").clicked() {
                        if self.audio_sink.empty() {
                            self.play_next_song();
                        } else {
                            self.audio_sink.play();
                        }
                    }
                } else if ui.button("||").clicked() {
                    self.audio_sink.pause();
                }

                ui.add(egui::ProgressBar::new(self.slider_value));
            });
        });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .min_width(10.0)
            .show(ctx, |ui| {
                self.show_library(ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            let request_play = self.playlist_view.show_playlist(
                ui,
                &mut self.playlist,
                &self.library,
                self.playlist_selected_song,
            );

            if let Some(entry) = request_play {
                self.play_playlist_entry(entry);
            }
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
