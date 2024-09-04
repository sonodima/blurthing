use iced::widget::button::{Appearance, StyleSheet};
use iced::{color, Border, Color, Shadow};

use super::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Default,
    Primary,
    Link,
}

impl StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        let background = match style {
            Button::Default => Some(self.palette.base_200.into()),
            Button::Primary => Some(self.palette.primary_100.into()),
            _ => None,
        };

        Appearance {
            shadow_offset: [0.0, 0.0].into(),
            background,
            text_color: match style {
                Button::Default => self.palette.base_content,
                Button::Primary => self.palette.primary_content,
                Button::Link => self.palette.primary_200,
            },
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_400,
                    Button::Primary => self.palette.primary_500,
                    _ => Color::TRANSPARENT,
                },
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: match style {
                    Button::Link => Color::TRANSPARENT,
                    _ => color!(0x000000, 0.3),
                },
                offset: [0.0, 0.0].into(),
                blur_radius: 8.0,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        let base = self.active(style);

        let background = match style {
            Button::Default => Some(self.palette.base_disabled.into()),
            Button::Primary => Some(self.palette.primary_disabled.into()),
            _ => None,
        };

        Appearance {
            background,
            text_color: match style {
                Button::Default => self.palette.base_content_disabled,
                Button::Primary => self.palette.primary_content_disabled,
                Button::Link => self.palette.primary_disabled,
            },
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_200,
                    Button::Primary => self.palette.primary_100,
                    _ => Color::TRANSPARENT,
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
            Button::Default => Some(self.palette.base_300.into()),
            Button::Primary => Some(self.palette.primary_200.into()),
            _ => None,
        };

        Appearance {
            background,
            text_color: match style {
                Button::Link => self.palette.primary_300,
                _ => base.text_color,
            },
            border: Border {
                color: match style {
                    Button::Default => self.palette.base_400,
                    Button::Primary => self.palette.primary_500,
                    _ => Color::TRANSPARENT,
                },
                ..base.border
            },
            shadow: Shadow {
                offset: [0.0, 3.0].into(),
                blur_radius: 12.0,
                ..base.shadow
            },
            ..base
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        let base = self.hovered(style);

        let background = match style {
            Button::Default => Some(self.palette.base_400.into()),
            Button::Primary => Some(self.palette.primary_300.into()),
            _ => None,
        };

        Appearance {
            background,
            text_color: match style {
                Button::Link => self.palette.primary_500,
                _ => base.text_color,
            },
            ..base
        }
    }
}
