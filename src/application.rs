use std::path::PathBuf;

use ::image::imageops::FilterType;
use ::image::{DynamicImage, GenericImageView, RgbaImage};
use anyhow::{anyhow, Result};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, image, slider, text, text_input};
use iced::widget::{column, mouse_area, row, scrollable};
use iced::{Application, Background, Border, Command, Element, Event, Length, Subscription, Theme};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use super::message::Message;
use super::parameters::Parameters;

pub const PREVIEW_SIZE: u32 = 512;
const IMAGE_DOWNSAMPLE_SIZE: u32 = 128;
const ALLOWED_EXTENSIONS: [&str; 8] = ["bmp", "gif", "jpg", "jpeg", "png", "tga", "tiff", "webp"];

#[derive(Default)]
pub struct BlurThing {
    img: Option<(PathBuf, DynamicImage)>,
    computed: Option<(String, DynamicImage)>,
    params: Parameters,
}

impl Application for BlurThing {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (BlurThing, Command<Self::Message>) {
        (BlurThing::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("BlurThing")
    }

    fn theme(&self) -> Self::Theme {
        Theme::TokyoNightLight
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
                    .unwrap_or_default();

                if ALLOWED_EXTENSIONS.contains(&extension) {
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

    fn view(&self) -> Element<Self::Message> {
        let left: Element<Self::Message> = if let Some((_, img)) = &self.computed {
            let buffer = img.to_rgba8().to_vec();
            let handle = image::Handle::from_pixels(img.width(), img.height(), buffer);
            image(handle).into()
        } else {
            let size = Length::Fixed(PREVIEW_SIZE as f32);
            container(
                text("Press on \"Select File\" or drop an image here to get started!")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center),
            )
            .height(size)
            .width(size)
            .padding(32)
            .style(self.no_image_style())
            .into()
        };

        let right = column![
            container(self.header()).style(self.container_style()),
            container(scrollable(self.controls()).height(Length::Fill))
                .style(self.container_style()),
            container(self.footer()).style(self.container_style())
        ];

        container(row![left, right]).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::event::listen_with(|event, _| match event {
            Event::Window(_, event) => match event {
                // Handle file drops in the application window.
                iced::window::Event::FileDropped(file) => Some(Message::FileDropped(file)),
                _ => None,
            },
            _ => None,
        })
    }
}

impl BlurThing {
    fn header(&self) -> Element<Message> {
        column![
            text(self.title()).size(24),
            mouse_area(text("Star ☆ me on GitHub").size(14)).on_press(Message::OpenProjectRepo)
        ]
        .width(Length::Fill)
        .padding(16)
        .into()
    }

    fn controls(&self) -> Element<Message> {
        let x_components = column![
            text("X Components"),
            text("Number of samples in the horizontal axis").size(12),
            slider(1..=8, self.params.components.0, Message::UpX),
        ];

        let y_components = column![
            text("Y Components"),
            text("Number of samples in the vertical axis").size(12),
            slider(1..=8, self.params.components.1, Message::UpY),
        ];

        let smoothness = column![
            text("Smoothness"),
            text("Amount of blur applied before the hash is computed").size(12),
            slider(0..=32, self.params.blur, Message::UpBlur),
        ];

        let hue_rotation = column![
            text("Hue Rotation"),
            text("How much to rotate the hue of the image (color shift)").size(12),
            slider(-180..=180, self.params.hue_rotate, Message::UpHue),
        ];

        let brightness = column![
            text("Brightness"),
            text("Adjusts the overall lightness or darkness of the image").size(12),
            slider(-100..=100, self.params.brightness, Message::UpBrightness),
        ];

        let contrast = column![
            text("Contrast"),
            text("Modifies the difference between the darkest and lightest parts of the image")
                .size(12),
            slider(-100..=100, self.params.contrast, Message::UpContrast),
        ];

        column![
            x_components,
            y_components,
            smoothness,
            hue_rotation,
            brightness,
            contrast,
        ]
        .padding(24)
        .spacing(8)
        .into()
    }

    fn footer(&self) -> Element<Message> {
        let hash_string = self
            .computed
            .as_ref()
            .map(|(hash, _)| hash.clone())
            .unwrap_or_default();

        let select_file = button(
            text("Select File")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::SelectImage);

        let out_hash = text_input("Load an image to compute its hash", &hash_string)
            .on_input(|_| Message::NoOp);

        let mut copy_to_clipboard = button("Copy to Clipboard");
        if self.computed.is_some() {
            copy_to_clipboard = copy_to_clipboard.on_press(Message::CopyHashToClipboard)
        }

        column![select_file, row![out_hash, copy_to_clipboard].spacing(8)]
            .padding(16)
            .spacing(8)
            .into()
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
        self.params = Parameters::default();
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

    fn no_image_style(&self) -> container::Appearance {
        let background = self.theme().extended_palette().background.strong.color;
        let border = self.theme().extended_palette().background.strong.text;
        container::Appearance {
            background: Some(Background::Color(background)),
            text_color: Some(border),
            ..Default::default()
        }
    }

    fn container_style(&self) -> container::Appearance {
        let border = self.theme().extended_palette().background.strong.color;
        container::Appearance {
            border: Border {
                color: border,
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
