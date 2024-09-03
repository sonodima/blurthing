use iced::widget::scrollable::{Appearance, Scrollbar, Scroller, StyleSheet};
use iced::Border;

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            container: Default::default(),
            scrollbar: Scrollbar {
                background: Some(self.palette.base_100.into()),
                border: Border::default(),
                scroller: Scroller {
                    color: self.palette.base_200,
                    border: Default::default(),
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
                        self.palette.base_300
                    } else {
                        self.palette.base_200
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
                    color: self.palette.base_400,
                    ..base.scrollbar.scroller
                },
                ..base.scrollbar
            },
            ..base
        }
    }
}
