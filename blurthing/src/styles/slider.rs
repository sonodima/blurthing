use iced::widget::slider::{Appearance, Handle, HandleShape, Rail, StyleSheet};
use iced::Color;

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            rail: Rail {
                colors: (self.palette.primary_200, self.palette.base_300),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: Handle {
                shape: HandleShape::Rectangle {
                    width: 6,
                    border_radius: 3.0.into(),
                },
                color: self.palette.primary_300,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        let base = self.active(_style);

        Appearance {
            rail: Rail {
                colors: (self.palette.primary_300, self.palette.base_400),
                ..base.rail
            },
            handle: Handle {
                color: self.palette.primary_500,
                ..base.handle
            },
            ..base
        }
    }

    fn dragging(&self, style: &Self::Style) -> Appearance {
        let base = self.hovered(style);

        Appearance {
            rail: Rail {
                colors: (self.palette.primary_500, self.palette.base_500),
                ..base.rail
            },
            ..base
        }
    }
}
