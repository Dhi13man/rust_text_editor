# Rust Text Editor

[![License](https://img.shields.io/github/license/dhi13man/rust_text_editor)](https://github.com/Dhi13man/rust_text_editor/blob/main/LICENSE)
[![Contributors](https://img.shields.io/github/contributors-anon/dhi13man/rust_text_editor?style=flat)](https://github.com/Dhi13man/rust_text_editor/graphs/contributors)
[![GitHub forks](https://img.shields.io/github/forks/dhi13man/rust_text_editor?style=social)](https://github.com/Dhi13man/rust_text_editor/network/members)
[![GitHub Repo stars](https://img.shields.io/github/stars/dhi13man/rust_text_editor?style=social)](https://github.com/Dhi13man/rust_text_editor)
[![Last Commit](https://img.shields.io/github/last-commit/dhi13man/rust_text_editor)](https://github.com/Dhi13man/rust_text_editor/commits/main)

Creation of a Rust-backed text editor that supports certain Vim like commands. Built in 6-ish as a part of a Hackathon.

## Installation and Contribution

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) SDK installed.

2. Clone this repository and CD into it.

3. Please view the [Contrbution Guidelines](https://raw.githubusercontent.com/Dhi13man/rust_text_editor/master/CONTRIBUTING.MD) here to know how you can contribute to this.

## Usage

1. Clone the repository and CD into it.

2. Execute `cargo run`.

3. Follow the commands in the Help Section. To open files, copy their path into your clipboard and hit `Ctrl + O`.

## Features

- Basic TUI Setup
- Proper Logs at all times
- User Friendly Help Menu
- Create Random Files
- Edit Files, Save Files, Open Multiple Files from the file paths saved in your Clipboard
- Word Wrap and Responsive UI
- Scrollable UI

## Inital Bugs and Open Issues (Feel Free to Contribute Fixes)

- [ ] Unoptimized: The Editor has virtually no optimization
- [ ] No Inuitive File Opener System: It can only open files when you copy the path to the files in your clipboard and hit the relevant shortcut
- [ ] A Mess: Code works, but is not organised too well
- [ ] Frontend: The UI could probably be improved

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
