use iced::widget::scrollable::{Appearance, Scrollbar, Scroller, StyleSheet};
use iced::Border;

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            container: Default::default(),
            scrollbar: Scrollbar {
                background: None,
                border: Border::default(),
                scroller: Scroller {
                    color: self.palette.base_300,
                    border: Border {
                        color: self.palette.base_500,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, _style: &Self::Style, is_mouse_over_scrollbar: bool) -> Appearance {
        let base = self.active(_style);

        Appearance {
            scrollbar: Scrollbar {
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar {
                        self.palette.base_400
                    } else {
                        self.palette.base_300
                    },
                    ..base.scrollbar.scroller
                },
                ..base.scrollbar
            },

            ..base
        }
    }

    fn dragging(&self, _style: &Self::Style) -> Appearance {
        let base = self.hovered(_style, true);

        Appearance {
            scrollbar: Scrollbar {
                scroller: Scroller {
                    color: self.palette.base_500,
                    ..base.scrollbar.scroller
                },
                ..base.scrollbar
            },
            ..base
        }
    }
}
