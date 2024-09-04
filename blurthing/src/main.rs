#![windows_subsystem = "windows"]

use iced::window;
use iced::{Application, Settings, Size};

use application::{BlurThing, PREVIEW_SIZE};

mod application;
mod message;
mod state;
mod styles;
mod undo_history;
mod utils;
mod widgets;

const ICON_DATA: &[u8] = include_bytes!("../../assets/icon/32x32@2x.png");

pub fn main() -> iced::Result {
    let icon = iced::window::icon::from_file_data(ICON_DATA, None);
    let window = window::Settings {
        size: Size::new(PREVIEW_SIZE as f32 * 2.0, PREVIEW_SIZE as f32),
        icon: icon.ok(),
        resizable: false,
        ..Default::default()
    };

    let settings = Settings {
        window,
        ..Default::default()
    };

    BlurThing::run(settings)
}
