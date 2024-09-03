use iced::widget::container::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Some(self.palette.base_100.into()),
            ..Default::default()
        }
    }
}
