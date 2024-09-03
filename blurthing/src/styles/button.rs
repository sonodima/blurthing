use iced::widget::button::{Appearance, StyleSheet};
use iced::{color, Border, Color, Shadow};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Default,
    Primary,
}

impl StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        let background = match style {
            Button::Default => self.palette.base_200.into(),
            Button::Primary => self.palette.primary_100.into(),
        };

        Appearance {
            shadow_offset: [0.0, 0.0].into(),
            background: Some(background),
            text_color: match style {
                Button::Default => self.palette.base_content,
                Button::Primary => self.palette.primary_content,
            },
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_400,
                    Button::Primary => self.palette.primary_500,
                },
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: color!(0x000000, 0.3),
                offset: [0.0, 0.0].into(),
                blur_radius: 8.0,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        let base = self.active(style);

        let background = match style {
            Button::Default => self.palette.base_disabled.into(),
            Button::Primary => self.palette.primary_disabled.into(),
        };

        Appearance {
            background: Some(background),
            text_color: match style {
                Button::Default => self.palette.base_content_disabled,
                Button::Primary => self.palette.primary_content_disabled,
            },
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_200,
                    Button::Primary => self.palette.primary_100,
                },
                ..base.border
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                ..Default::default()
            },
            ..base
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let base = self.active(style);

        let background = match style {
            Button::Default => self.palette.base_300.into(),
            Button::Primary => self.palette.primary_200.into(),
        };

        Appearance {
            background: Some(background),
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_400,
                    Button::Primary => self.palette.primary_500,
                },
                ..base.border
            },
            shadow: Shadow {
                color: color!(0x000000, 0.3),
                offset: [0.0, 3.0].into(),
                blur_radius: 12.0,
            },
            ..base
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        let base = self.hovered(style);

        let background = match style {
            Button::Default => self.palette.base_400.into(),
            Button::Primary => self.palette.primary_300.into(),
        };

        Appearance {
            background: Some(background),
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_400,
                    Button::Primary => self.palette.primary_500,
                },
                ..base.border
            },
            ..base
        }
    }
}
