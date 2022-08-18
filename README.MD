# Rust Text Editor

Creation of a Rust-backed text editor that supports certain Vim like commands. Built as a part of a Hackathon.

## Installation and Contribution

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) SDK installed.

2. Clone this repository and CD into it.

3. Please view the [Contrbution Guidelines](https://raw.githubusercontent.com/Dhi13man/rust_text_editor/master/CONTRIBUTING.MD) here to know how you can contribute to this.

## Usage

1. Clone the repository and CD into it.

2. Execute `cargo run`.

3. Follow the commands in the Help Section. To open files, copy their path into your clipboard and hit `Ctrl + O`.

## Features

- Basic TUI Setup.
- Proper Logs at all times
- User Friendly Help Menu
- Create Random Files
- Open Multiple Files from the file paths saved in your Clipboard
- Edit Files, Save Files
- Word Wrap and Responsive UI
- Scrollable UI

## Acknowledgement

### ilaborie/plop-tui

This project is built on top of [plop-tui](https://github.com/ilaborie/plop-tui) to implement generic features of a CLI based Code editor.

### Dependencies

The project utilizes various open-source Cargo Crate dependencies to work. Big thanks to all the creators.

```toml
log = "0.4"

copypasta = "0.8.1"

names = { version = "0.14.0", default-features = false }

tui-logger = "0.8.0"
tui = "0.18.0"

crossterm = "0.24.0"

tokio = { version = "1", features = ["full"] }

eyre = "0.6"
```

## Screenshots

| Editor Initialized |
| :---: |
| ![Editor Initialized](https://raw.githubusercontent.com/Dhi13man/rust_text_editor/master/assets/screenshots/Editor_Initialized.png) |

| Editor Write Mode |
| :---: |
| ![Editor Write Mode](https://raw.githubusercontent.com/Dhi13man/rust_text_editor/master/assets/screenshots/Editor_Write_Mode.png) |

| Full Functionality With Multiple Files |
| :---: |
| ![Full Functionality With Multiple Files](https://raw.githubusercontent.com/Dhi13man/rust_text_editor/master/assets/screenshots/Full_Functionality_Multiple_Files.png) |
