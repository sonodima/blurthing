use std::path::PathBuf;

use ::image::DynamicImage;
use anyhow::Result;

#[derive(Debug)]
pub enum Message {
    Interaction(Interaction),

    FileDropped(PathBuf),
    ImageDownloaded(Result<DynamicImage>),
}

#[derive(Debug, Clone)]
pub enum Interaction {
    Undo,
    Redo,

    SelectImage,
    ExportImage,

    CopyToClipboard,
    ImFeelingLucky,
    OpenProjectRepo,

    SaveParameters,
    UpX(u32),
    UpY(u32),
    UpBlur(i32),
    UpHue(i32),
    UpBrightness(i32),
    UpContrast(i32),

    // Command to do nothing, useful to enable TextInputs even when read only.
    Ignored,
}
