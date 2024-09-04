#![windows_subsystem = "windows"]

use iced::{window, Application, Settings, Size};

use application::{BlurThing, PREVIEW_SIZE};

mod application;
mod message;
mod state;
mod styles;
mod undo_history;
mod utils;
mod widgets;

pub fn main() -> iced::Result {
    let window = window::Settings {
        size: Size::new(PREVIEW_SIZE as f32 * 2.0, PREVIEW_SIZE as f32),
        resizable: false,
        ..Default::default()
    };

    let settings = Settings {
        window,
        ..Default::default()
    };

    BlurThing::run(settings)
}
