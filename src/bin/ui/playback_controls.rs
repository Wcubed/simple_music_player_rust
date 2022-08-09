use crate::egui::Response;
use eframe::egui::{ImageButton, Slider, Ui, Widget};
use egui_extras::RetainedImage;

pub struct PlaybackControls {
    icons: PlaybackIcons,
}

impl PlaybackControls {
    pub fn new() -> Self {
        PlaybackControls {
            icons: PlaybackIcons::new(),
        }
    }

    pub fn show(&self, ui: &mut Ui, paused: bool, volume: i64) -> Option<PlaybackCommand> {
        let mut command = None;

        // TODO: implement actual playback controls.
        if image_button(ui, &self.icons.previous_song).clicked() {
            command = Some(PlaybackCommand::PreviousSong);
        }

        if paused {
            if image_button(ui, &self.icons.play).clicked() {
                command = Some(PlaybackCommand::Unpause);
            }
        } else if image_button(ui, &self.icons.pause).clicked() {
            command = Some(PlaybackCommand::Pause);
        }

        if image_button(ui, &self.icons.next_song).clicked() {
            command = Some(PlaybackCommand::NextSong);
        }

        match volume {
            0..=33 => {
                self.icons.volume_low.show(ui);
            }
            34..=66 => {
                self.icons.volume_mid.show(ui);
            }
            _ => {
                self.icons.volume_high.show(ui);
            }
        }

        let mut new_volume = volume;
        ui.add(Slider::new(&mut new_volume, 0..=100).show_value(false));

        if new_volume != volume {
            command = Some(PlaybackCommand::SetVolume(new_volume));
        }

        command
    }
}

fn image_button(ui: &mut Ui, image: &RetainedImage) -> Response {
    ImageButton::new(image.texture_id(ui.ctx()), image.size_vec2()).ui(ui)
}

#[derive(Copy, Clone)]
pub enum PlaybackCommand {
    Pause,
    Unpause,
    NextSong,
    PreviousSong,
    SetVolume(i64),
}

struct PlaybackIcons {
    pub play: RetainedImage,
    pub pause: RetainedImage,
    pub next_song: RetainedImage,
    pub previous_song: RetainedImage,
    pub volume_low: RetainedImage,
    pub volume_mid: RetainedImage,
    pub volume_high: RetainedImage,
}

impl PlaybackIcons {
    fn new() -> Self {
        PlaybackIcons {
            play: RetainedImage::from_svg_bytes(
                "play.svg",
                include_bytes!("../../../assets/icons/play.svg"),
            )
            .unwrap(),
            pause: RetainedImage::from_svg_bytes(
                "pause.svg",
                include_bytes!("../../../assets/icons/pause.svg"),
            )
            .unwrap(),
            next_song: RetainedImage::from_svg_bytes(
                "next_song.svg",
                include_bytes!("../../../assets/icons/next_song.svg"),
            )
            .unwrap(),
            previous_song: RetainedImage::from_svg_bytes(
                "previous_song.svg",
                include_bytes!("../../../assets/icons/previous_song.svg"),
            )
            .unwrap(),
            volume_low: RetainedImage::from_svg_bytes(
                "volume_low.svg",
                include_bytes!("../../../assets/icons/volume_low.svg"),
            )
            .unwrap(),
            volume_mid: RetainedImage::from_svg_bytes(
                "volume_mid.svg",
                include_bytes!("../../../assets/icons/volume_mid.svg"),
            )
            .unwrap(),
            volume_high: RetainedImage::from_svg_bytes(
                "volume_low.svg",
                include_bytes!("../../../assets/icons/volume_high.svg"),
            )
            .unwrap(),
        }
    }
}
