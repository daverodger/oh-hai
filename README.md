# oh-hai

---
A terminal command bookmarking tool written in Rust using [ratatui](https://github.com/ratatui-org/ratatui).

Heavily inspired by [tbmk](https://github.com/linhx/tbmk).

![oh-hai demo](./assets/demo.gif)

## Usage

---

- search (get): `ctrl + g`
- delete: `ctrl + d`
- insert (bookmark): `ctrl + b`

The command-line buffer is copied into the search field (search mode) or the command field (insert mode).

## Build and Install

---
You'll need the rustc and cargo which are best installed using [rustup](https://www.rust-lang.org/tools/install)

1. Run `./build` and move the newly generated `/oh_hai` directory to its new home
1. cd into the app directory and run `./install`
1. Restart shell or source config file

Bookmarks are saved into `./data/bookmarks.json` for manual editing.

If you need to move the app directory, run `./install` again after moving.

## Todo

---
- Additional shell support (currently bash only)
- Any testing at all would be an improvement