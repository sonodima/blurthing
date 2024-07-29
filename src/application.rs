use std::path::PathBuf;

use ::image::imageops::FilterType;
use ::image::{DynamicImage, RgbaImage};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, column, container, image, mouse_area, row, scrollable, slider, text, text_input,
};
use iced::{
    event, executor, window, Application, Background, Border, Color, Command, Element, Event,
    Length, Subscription, Theme,
};

use super::message::Message;
use super::parameters::Parameters;

pub const PREVIEW_SIZE: u32 = 512;
const IMAGE_DOWNSAMPLE_SIZE: u32 = 128;

#[derive(Default)]
pub struct BlurThing {
    image: Option<(PathBuf, DynamicImage)>,
    computed: Option<(String, DynamicImage)>,
    params: Parameters,
}

impl BlurThing {
    fn try_load_image(&mut self, path: PathBuf) {
        if let Ok(loaded) = ::image::open(&path) {
            let resized = loaded.resize_exact(
                IMAGE_DOWNSAMPLE_SIZE,
                IMAGE_DOWNSAMPLE_SIZE,
                FilterType::Lanczos3,
            );

            self.image = Some((path, resized));
            self.params = Parameters::default();
            self.compute_blurhash();
        } else {
            eprintln!("failed to load image");
        }
    }

    fn compute_blurhash(&mut self) {
        if let Some(image) = &self.image {
            let buffer = image
                .1
                .blur(self.params.blur as f32)
                .huerotate(self.params.hue_rotate)
                .adjust_contrast(self.params.contrast as f32)
                .brighten(self.params.brightness * 2)
                .to_rgba8()
                .to_vec();

            if let Ok(hash) = blurhash::encode(
                self.params.components.0,
                self.params.components.1,
                image.1.width(),
                image.1.height(),
                &buffer,
            ) {
                let decoded = blurhash::decode(&hash, PREVIEW_SIZE, PREVIEW_SIZE, 1.0).unwrap();
                let preview = RgbaImage::from_vec(PREVIEW_SIZE, PREVIEW_SIZE, decoded).unwrap();
                self.computed = Some((hash, DynamicImage::ImageRgba8(preview)));
            } else {
                eprintln!("failed to compute blurhash for the image");
            }
        }
    }

    fn placeholder_style() -> container::Appearance {
        let color = Background::Color(Color::from_rgb(0.9, 0.9, 0.9));
        container::Appearance {
            background: Some(color),
            ..Default::default()
        }
    }

    fn container_style() -> container::Appearance {
        container::Appearance {
            border: Border {
                color: Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Application for BlurThing {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (BlurThing, Command<Self::Message>) {
        (BlurThing::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("BlurThing")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SelectImage => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg", "webp"])
                    .pick_file()
                {
                    self.try_load_image(path);
                }
            }
            Message::OnFileDropped(path) => {
                let extension = path.extension().unwrap().to_str().unwrap();
                if ["png", "jpg", "jpeg", "webp"].contains(&extension) {
                    self.try_load_image(path);
                }
            }
            Message::CopyHashToClipboard => {
                if let Some((hash, _)) = &self.computed {
                    return iced::clipboard::write(hash.clone());
                }
            }

            Message::UpX(x) => {
                self.params.components.0 = x;
                self.compute_blurhash();
            }
            Message::UpY(y) => {
                self.params.components.1 = y;
                self.compute_blurhash();
            }
            Message::UpBlur(blur) => {
                self.params.blur = blur;
                self.compute_blurhash();
            }

            Message::UpHue(hue) => {
                self.params.hue_rotate = hue;
                self.compute_blurhash();
            }
            Message::UpBrightness(brightness) => {
                self.params.brightness = brightness;
                self.compute_blurhash();
            }
            Message::UpContrast(contrast) => {
                self.params.contrast = contrast;
                self.compute_blurhash();
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
            .style(Self::placeholder_style())
            .into()
        };

        let x_control = column![
            text("X Components"),
            text("Number of samples in the horizontal axis").size(12),
            slider(1..=8, self.params.components.0, Message::UpX),
        ];

        let y_control = column![
            text("Y Components"),
            text("Number of samples in the vertical axis").size(12),
            slider(1..=8, self.params.components.1, Message::UpY),
        ];

        let blur_control = column![
            text("Smoothness"),
            text("Amount of blur applied before the hash is computed").size(12),
            slider(0..=32, self.params.blur, Message::UpBlur),
        ];

        let hue_control = column![
            text("Hue Rotation"),
            text("How much to rotate the hue of the image (color shift)").size(12),
            slider(-180..=180, self.params.hue_rotate, Message::UpHue),
        ];

        let brightness_control = column![
            text("Brightness"),
            text("Adjusts the overall lightness or darkness of the image").size(12),
            slider(-100..=100, self.params.brightness, Message::UpBrightness),
        ];

        let contrast_control = column![
            text("Contrast"),
            text("Modifies the difference between the darkest and lightest parts of the image")
                .size(12),
            slider(-100..=100, self.params.contrast, Message::UpContrast),
        ];

        let select_file_button = button(
            text("Select File")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(Length::Fill)
        .on_press(Message::SelectImage);

        let hash_data = self
            .computed
            .as_ref()
            .map(|(hash, _)| hash.clone())
            .unwrap_or_default();
        let hash_field = mouse_area(
            text_input("Load an image to compute its hash", &hash_data)
                .on_input(|_| Message::NoOp)
                .width(Length::Fill),
        )
        .on_press(Message::CopyHashToClipboard);

        let copy_control = button(text("Copy to Clipboard")).on_press(Message::CopyHashToClipboard);

        let header = column![
            text("BlurThing").size(24),
            text("github.com / sonodima / blurthing").size(14),
        ]
        .width(Length::Fill)
        .padding(16);

        let footer = column![
            row![hash_field, copy_control].spacing(8),
            select_file_button
        ]
        .padding(16)
        .spacing(8);

        let controls = column![
            x_control,
            y_control,
            blur_control,
            hue_control,
            brightness_control,
            contrast_control,
        ]
        .padding(24)
        .spacing(8);

        let right = column![
            container(header).style(Self::container_style()),
            container(scrollable(controls).height(Length::Fill)).style(Self::container_style()),
            container(footer).style(Self::container_style())
        ];

        container(row![left, right]).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, _| match event {
            Event::Window(_, event) => match event {
                window::Event::FileDropped(file) => Some(Message::OnFileDropped(file)),
                _ => None,
            },
            _ => None,
        })
    }
}
