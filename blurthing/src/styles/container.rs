use iced::widget::container::{Appearance, StyleSheet};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Light,
    Medium,
}

impl StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let background = match style {
            Container::Light => self.palette.base_100.into(),
            Container::Medium => self.palette.base_200.into(),
        };

        Appearance {
            background: Some(background),
            ..Default::default()
        }
    }
}
