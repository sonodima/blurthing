<div align="center" style="margin-top: 24px;">
    <div>
        <h1>BlurThing 🌄</h1>
        <span>Simple application for generating and customizing <a href="https://blurha.sh">
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
    <br/><br/>
    <img src="assets/demo.gif" alt="Application demo" width="80%" />
</div>

## Usage

1. Open an image file with the open file dialog
2. Adjust the image manipulation parameters to your liking
3. Copy the blur hash to the clipboard

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

```sh
xattr -d com.apple.quarantine /path/to/BlurThing.app
```

_(replacing `/path/to/BlurThing.app` with the actual path to the application)_

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
