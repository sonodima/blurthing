use crate::styles::Theme;

// Export iced's native widgets with our custom theme. When you need to use a
// widget in your application, you should use this module instead of iced's.

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Theme>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Theme>;
pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Theme>;

pub type Button<'a, Message> = iced::widget::Button<'a, Message, Theme>;
pub type Slider<'a, T, Message> = iced::widget::Slider<'a, T, Message, Theme>;
pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, Theme>;
pub type Text<'a> = iced::widget::Text<'a, Theme>;
