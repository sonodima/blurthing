use iced::widget::text::{Appearance, StyleSheet};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    #[default]
    Regular,
    Subtle,
}

impl StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> Appearance {
        Appearance {
            color: match style {
                Text::Regular => None,
                Text::Subtle => Some(self.palette.base_content_subtle),
            },
        }
    }
}
