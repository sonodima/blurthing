<div align="center" style="margin-top: 24px;">
    <div>
        <h1>BlurThing 🌄</h1>
        <span>Cross-platform application for generating and customizing <a href="https://blurha.sh">
            BlurHashes</a> from<br/>images, with extensive controls for
            image manipulation.</span>
    </div>
    <br/>
    <a href="https://github.com/sonodima/blurthing/releases/latest"> 
        <img src="https://img.shields.io/github/v/release/sonodima/blurthing?style=for-the-badge&color=yellow"/>
    </a>
    <a href="https://github.com/sonodima/blurthing/actions/workflows/ci.yml"> 
        <img src="https://img.shields.io/github/actions/workflow/status/sonodima/blurthing/ci.yml?style=for-the-badge&label=CI%20Status"/>
    </a>
    <a href="LICENSE"> 
        <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge"/>
    </a>
    <a href="https://github.com/iced-rs/iced">
        <img alt="Made with Iced" src="assets/made-with-iced.svg" height="28px" />
    </a>
</div>

## Basic Usage

1. Open an image file with the open file dialog
2. Adjust the image manipulation parameters to your liking
3. Copy the blur hash to the clipboard, or alternatively export the blurred rastered image

![Application Demo](assets/demo.gif)

## How is this useful?

BlurHashes are typically used in scenarios where you want to display a placeholder image while it is being loaded.

If that is your use case, this application can be a quick way to generate a BlurHash for your images. _(for testing, or if you are lazy and didn't integrate the BlurHash generation in your backend)_

However, BlurThing can be used to manipulate an input image and export a blurred version of it.
With the right parameters you can create some cool f\*\*king abstract wallpapers that look like they came straight from the iPhone 19.

BlurThing can be helpful, or it can be a fun toy. It's up to you.

## Hotkeys

Some hotkeys are available for faster interaction:

- <kbd>Ctrl</kbd> + <kbd>O</kbd> -> shows the open file dialog
- <kbd>Ctrl</kbd> + <kbd>C</kbd> -> copies the BlurHash to the clipboard
- <kbd>Ctrl</kbd> + <kbd>S</kbd> -> exports the current blurred image to a file
- <kbd>Ctrl</kbd> + <kbd>Z</kbd> -> undo the last modification
- <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Z</kbd> -> redo the last modification

**Note**: On macOS, replace <kbd>Ctrl</kbd> with <kbd>Command</kbd>

## Getting Started

The easiest way to run BlurThing is by using the pre-built binaries available in the
[releases](https://github.com/sonodima/blurthing/releases/latest) page.

From there, you want to download the appropriate installer for your operating system
and architecture.

---

**Note**: If you are using macOS, you may be greeted with a _"BlurThing can't be opened"_
message. This is due to the application currently not being signed.

To open it anyway, you can right-click on the application and select **Open**.

## Build and Run

To compile the main executable, run:

```sh
cargo run --bin blurhash --release
```

### Packaging and Bundling

With the help of the `tauri-bundler` crate, it is possible to automagically create
OS-specific distribution bundles.

To build and bundle the application, run:

```sh
cargo xtask bundle --release
```

## Future Plans and Ideas

- [ ] Add new image manipulation filters (e.g. rotation, flip, waves???, distort???)
- [ ] Improve the export UX, allowing the user to choose the output resolution
- [x] Implement an _"I feel lucky"_ button that loads a random image from the internet _(what could go wrong anyway?)_
