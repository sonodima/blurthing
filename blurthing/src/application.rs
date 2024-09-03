use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use ::image::imageops::FilterType;
use ::image::{DynamicImage, GenericImageView, RgbaImage};
use anyhow::{anyhow, Result};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::Image;

use iced::{Application, Command, Event, Length, Subscription};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use super::message::Message;
use super::parameters::Parameters;
use super::styles::Theme;
use super::undo_history::UndoHistory;
use crate::{styles, widgets::*};

pub const PREVIEW_SIZE: u32 = 512;
const IMAGE_DOWNSAMPLE_SIZE: u32 = 128;
const ALLOWED_EXTENSIONS: [&str; 8] = ["bmp", "gif", "jpg", "jpeg", "png", "tga", "tiff", "webp"];
const EXPORT_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

pub struct BlurThing {
    img: Option<(PathBuf, DynamicImage)>,
    computed: Option<(String, DynamicImage)>,

    params: Parameters,
    history: UndoHistory<Parameters>,
}

impl Application for BlurThing {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut instance = Self {
            img: None,
            computed: None,

            params: Default::default(),
            history: UndoHistory::new(),
        };

        instance.reset_settings();
        (instance, Command::none())
    }

    fn title(&self) -> String {
        String::from("BlurThing")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SelectImage => {
                if let Ok(Some(path)) = FileDialog::new()
                    .add_filter("Image File", &ALLOWED_EXTENSIONS)
                    .show_open_single_file()
                {
                    if let Err(e) = self.try_load_image(path) {
                        eprintln!("image load failed: {}", e);
                        _ = MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Image Load Error")
                            .set_text(&format!("failed to load image: {}", e))
                            .show_alert();
                    }
                }
            }
            Message::FileDropped(path) => {
                let extension = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_lowercase();

                if ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
                    if let Err(e) = self.try_load_image(path) {
                        eprintln!("image load failed: {}", e);
                        _ = MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Image Load Error")
                            .set_text(&format!("failed to load image: {}", e))
                            .show_alert();
                    }
                } else {
                    eprintln!("received unsupported file type: {}", extension);
                    _ = MessageDialog::new()
                        .set_type(MessageType::Warning)
                        .set_title("Unsupported File Type")
                        .set_text("the dropped file does not appear to be a supported image type")
                        .show_alert();
                }
            }
            Message::CopyHashToClipboard => {
                if let Some((hash, _)) = &self.computed {
                    return iced::clipboard::write(hash.clone());
                }
            }
            Message::ExportImage => {
                if let Some((_, img)) = &self.computed {
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    if let Ok(Some(path)) = FileDialog::new()
                        .add_filter("Image File", &EXPORT_EXTENSIONS)
                        .set_filename(&format!("blurthing-{}.jpg", timestamp))
                        .show_save_single_file()
                    {
                        if let Err(e) = img.clone().into_rgb8().save(path) {
                            eprintln!("image export failed: {}", e);
                            _ = MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("Image Export Error")
                                .set_text(&format!(
                                    "failed to export image: {}",
                                    e.to_string().to_lowercase()
                                ))
                                .show_alert();
                        }
                    }
                }
            }
            Message::OpenProjectRepo => {
                let url = env!("CARGO_PKG_REPOSITORY");
                if let Err(e) = webbrowser::open(url) {
                    eprintln!("failed to open project repository: {}", e);
                    _ = MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Application Error")
                        .set_text(&format!("failed to open project repository: {}", e))
                        .show_alert();
                }
            }

            Message::SaveParameters => {
                self.history.push(self.params.clone());
            }
            Message::Undo => {
                if let Some(params) = self.history.undo() {
                    self.params = params.clone();
                    self.compute_blurhash_checked();
                }
            }
            Message::Redo => {
                if let Some(params) = self.history.redo() {
                    self.params = params.clone();
                    self.compute_blurhash_checked();
                }
            }

            Message::UpX(x) => {
                self.params.components.0 = x;
                self.compute_blurhash_checked();
            }
            Message::UpY(y) => {
                self.params.components.1 = y;
                self.compute_blurhash_checked();
            }
            Message::UpBlur(blur) => {
                self.params.blur = blur;
                self.compute_blurhash_checked();
            }

            Message::UpHue(hue) => {
                self.params.hue_rotate = hue;
                self.compute_blurhash_checked();
            }
            Message::UpBrightness(brightness) => {
                self.params.brightness = brightness;
                self.compute_blurhash_checked();
            }
            Message::UpContrast(contrast) => {
                self.params.contrast = contrast;
                self.compute_blurhash_checked();
            }

            Message::NoOp => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::event::listen_with(|event, _| {
            match event {
                Event::Window(_, event) => match event {
                    // Handle file drops in the application window.
                    iced::window::Event::FileDropped(file) => Some(Message::FileDropped(file)),
                    _ => None,
                },
                Event::Keyboard(event) => match event {
                    // Handle application hotkeys (when the command / control key is pressed).
                    iced::keyboard::Event::KeyPressed { key, modifiers, .. }
                        if modifiers.command() =>
                    {
                        Self::handle_hotkey(key, modifiers)
                    }
                    _ => None,
                },
                _ => None,
            }
        })
    }

    fn view(&self) -> Element<Self::Message> {
        let right = Column::new()
            .push(Container::new(self.header()).width(Length::Fill))
            .push(Scrollable::new(self.controls()).height(Length::Fill))
            .push(Container::new(self.footer()));

        Row::new().push(self.preview()).push(right).into()
    }
}

//
// Main Logic
//

impl BlurThing {
    fn handle_hotkey(
        key: iced::keyboard::Key,
        modifiers: iced::keyboard::Modifiers,
    ) -> Option<Message> {
        match key.as_ref() {
            iced::keyboard::Key::Character("o") => Some(Message::SelectImage),
            iced::keyboard::Key::Character("c") => Some(Message::CopyHashToClipboard),
            iced::keyboard::Key::Character("s") => Some(Message::ExportImage),
            iced::keyboard::Key::Character("z") => {
                if modifiers.shift() {
                    Some(Message::Redo)
                } else {
                    Some(Message::Undo)
                }
            }
            _ => None,
        }
    }

    fn try_load_image(&mut self, path: PathBuf) -> Result<()> {
        let loaded = ::image::open(&path).map_err(|e| anyhow!(e.to_string().to_lowercase()))?;

        // Downsample the image to a smaller size for faster processing.
        let resized = loaded.resize_exact(
            IMAGE_DOWNSAMPLE_SIZE,
            IMAGE_DOWNSAMPLE_SIZE,
            FilterType::Lanczos3,
        );

        // Store the image and reset the parameters to their defaults.
        self.img = Some((path, resized));
        self.reset_settings();
        self.compute_blurhash()
    }

    fn compute_blurhash(&mut self) -> Result<()> {
        let img = self
            .img
            .as_ref()
            .ok_or_else(|| anyhow!("source image is not available"))?;

        let buffer = img
            .1
            .blur(self.params.blur as f32)
            .huerotate(self.params.hue_rotate)
            .adjust_contrast(self.params.contrast as f32)
            .brighten(self.params.brightness * 2)
            .to_rgba8()
            .to_vec();

        let (width, height) = img.1.dimensions();
        let (x, y) = self.params.components;
        // Encode the blurhash and decode it to a preview image for display.
        let hash = blurhash::encode(x, y, width, height, &buffer)
            .map_err(|_| anyhow!("failed to compute the blurhash"))?;
        let decoded = blurhash::decode(&hash, PREVIEW_SIZE, PREVIEW_SIZE, 1.0)
            .map_err(|_| anyhow!("failed to decode the computed blurhash"))?;
        let preview = RgbaImage::from_vec(PREVIEW_SIZE, PREVIEW_SIZE, decoded)
            .ok_or_else(|| anyhow!("failed to create preview image from decoded buffer"))?;

        self.computed = Some((hash, DynamicImage::ImageRgba8(preview)));
        Ok(())
    }

    fn compute_blurhash_checked(&mut self) {
        if self.img.is_none() {
            return;
        }

        if let Err(e) = self.compute_blurhash() {
            eprintln!("failed to compute blurhash: {}", e);
            _ = MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Computation Error")
                .set_text(&format!("failed to compute blurhash: {}", e))
                .show_alert();
        }
    }

    fn reset_settings(&mut self) {
        self.params = Parameters::default();
        self.history.reset();
        // Push the initial parameters to the history stack.
        self.history.push(self.params.clone());
    }
}

//
// UI Components
//

impl BlurThing {
    fn preview(&self) -> Element<Message> {
        if let Some((_, img)) = &self.computed {
            let handle = iced::widget::image::Handle::from_pixels(
                img.width(),
                img.height(),
                img.to_rgba8().to_vec(),
            );

            Image::new(handle).into()
        } else {
            Container::new(
                Text::new("Press on \"Select File\" or drop an image here to get started")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            )
            .style(styles::Container::Medium)
            .height(Length::Fixed(PREVIEW_SIZE as f32))
            .width(Length::Fixed(PREVIEW_SIZE as f32))
            .padding(32)
            .into()
        }
    }

    fn header(&self) -> Element<Message> {
        MouseArea::new(
            Column::new()
                .push(Text::new(self.title()).size(24))
                .push(Text::new("by sonodima").size(14))
                .padding([16, 24]),
        )
        .on_press(Message::OpenProjectRepo)
        .into()
    }

    fn controls(&self) -> Element<Message> {
        let x_components = Column::new()
            .push(Text::new("X Components"))
            .push(
                Text::new("Number of samples in the horizontal axis")
                    .style(styles::Text::Subtle)
                    .size(12),
            )
            .push(
                Slider::new(1..=8, self.params.components.0, Message::UpX)
                    .on_release(Message::SaveParameters),
            );

        let y_components = Column::new()
            .push(Text::new("Y Components"))
            .push(
                Text::new("Number of samples in the vertical axis")
                    .style(styles::Text::Subtle)
                    .size(12),
            )
            .push(
                Slider::new(1..=8, self.params.components.1, Message::UpY)
                    .on_release(Message::SaveParameters),
            );

        let smoothness = Column::new()
            .push(Text::new("Smoothness"))
            .push(
                Text::new("Amount of blur applied before the hash is computed")
                    .style(styles::Text::Subtle)
                    .size(12),
            )
            .push(
                Slider::new(0..=32, self.params.blur, Message::UpBlur)
                    .on_release(Message::SaveParameters),
            );

        let hue_rotation = Column::new()
            .push(Text::new("Hue Rotation"))
            .push(
                Text::new("How much to rotate the hue of the image (color shift)")
                    .style(styles::Text::Subtle)
                    .size(12),
            )
            .push(
                Slider::new(-180..=180, self.params.hue_rotate, Message::UpHue)
                    .on_release(Message::SaveParameters),
            );

        let brightness = Column::new()
            .push(Text::new("Brightness"))
            .push(
                Text::new("Adjusts the overall lightness or darkness of the image")
                    .style(styles::Text::Subtle)
                    .size(12),
            )
            .push(
                Slider::new(-100..=100, self.params.brightness, Message::UpBrightness)
                    .on_release(Message::SaveParameters),
            );

        let contrast = Column::new()
            .push(Text::new("Contrast"))
            .push(
                Text::new(
                    "Modifies the difference between the darkest and lightest parts of the image",
                )
                .style(styles::Text::Subtle)
                .size(12),
            )
            .push(
                Slider::new(-100..=100, self.params.contrast, Message::UpContrast)
                    .on_release(Message::SaveParameters),
            );

        Column::new()
            .push(x_components)
            .push(y_components)
            .push(smoothness)
            .push(hue_rotation)
            .push(brightness)
            .push(contrast)
            .padding(24)
            .spacing(8)
            .into()
    }

    fn footer(&self) -> Element<Message> {
        let select_file = Button::new(
            Text::new("Select File")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::SelectImage);

        let mut export_image = Button::new("Export Image").style(styles::Button::Primary);
        if self.computed.is_some() {
            export_image = export_image.on_press(Message::ExportImage)
        }

        let hash_string = self
            .computed
            .as_ref()
            .map(|(hash, _)| hash.clone())
            .unwrap_or_default();
        let mut out_hash = TextInput::new("Load an image to compute its hash", &hash_string);
        if self.computed.is_some() {
            out_hash = out_hash.on_input(|_| Message::NoOp);
        }

        let mut copy_to_clipboard = Button::new("Copy to Clipboard").style(styles::Button::Primary);
        if self.computed.is_some() {
            copy_to_clipboard = copy_to_clipboard.on_press(Message::CopyHashToClipboard)
        }

        Column::new()
            .push(Row::new().push(select_file).push(export_image).spacing(8))
            .push(Row::new().push(out_hash).push(copy_to_clipboard).spacing(8))
            .padding(16)
            .spacing(8)
            .into()
    }
}
