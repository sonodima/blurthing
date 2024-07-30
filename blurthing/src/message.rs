use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,

    SelectImage,
    FileDropped(PathBuf),
    CopyHashToClipboard,
    OpenProjectRepo,

    SaveParameters,
    Undo,
    Redo,

    UpX(u32),
    UpY(u32),
    UpBlur(i32),

    UpHue(i32),
    UpBrightness(i32),
    UpContrast(i32),
}
