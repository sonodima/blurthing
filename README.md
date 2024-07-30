<div align="center" style="margin-top: 24px;">
    <img src="assets/demo.gif" alt="Application demo" width="80%" />
    <div style="margin-top: 24px; margin-bottom: 24;">
        <h1>BlurThing 🌄</h1>
        <span>Simple application for generating and customizing <a href="https://blurha.sh">
            BlurHashes</a> from<br/>images, with extensive controls for
            image manipulation.</span>
    </div>
</div>

## Usage

1. Open an image file with the open file dialog
2. Adjust the image manipulation parameters to your liking
3. Copy the blur hash to the clipboard

## Hotkeys

Some hotkeys are available for faster interaction:

- <kbd>Ctrl</kbd> + <kbd>O</kbd> -> shows the open file dialog
- <kbd>Ctrl</kbd> + <kbd>C</kbd> -> copies the BlurHash to the clipboard
- <kbd>Ctrl</kbd> + <kbd>Z</kbd> -> undo the last modification
- <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Z</kbd> -> redo the last modification

## Build and Run

### Building

To compile the main executable, run:

```sh
cargo run --release
```

### Packaging

After building the executable, you can bundle it into an OS-specific package
using the following command:

```sh
cargo xtask bundle
```
