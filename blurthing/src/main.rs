#![windows_subsystem = "windows"]

use iced::{window, Application, Settings, Size};

use application::{BlurThing, PREVIEW_SIZE};

mod application;
mod message;
mod parameters;

pub fn main() -> iced::Result {
    let window = window::Settings {
        size: Size::new(1024.0, PREVIEW_SIZE as f32),
        resizable: false,
        ..Default::default()
    };

    let settings = Settings {
        window,
        ..Default::default()
    };

    BlurThing::run(settings)
}
