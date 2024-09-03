use iced::application::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background_color: self.palette.background,
            text_color: self.palette.foreground,
        }
    }
}
