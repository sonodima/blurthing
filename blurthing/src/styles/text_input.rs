use iced::widget::text_input::{Appearance, StyleSheet};
use iced::{Background, Border, Color};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: self.palette.base_200.into(),
            border: Border {
                color: self.palette.base_400,
                width: 1.0,
                radius: 6.0.into(),
            },
            icon_color: self.palette.base_content,
        }
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: self.palette.base_300.into(), // TODO: change
            ..self.active(_style)
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        self.active(_style)
    }

    fn focused(&self, _style: &Self::Style) -> Appearance {
        let base = self.active(_style);

        Appearance {
            border: Border {
                color: self.palette.primary_200,
                ..base.border
            },
            ..base
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.palette.base_400 // todo change
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette.base_400
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        let mut color = self.palette.primary_100;
        color.a = 0.2;
        color
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette.base_content
    }
}
