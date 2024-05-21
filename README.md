# oh-hai

---
A terminal command bookmarking tool written in Rust using [ratatui](https://github.com/ratatui-org/ratatui).

Heavily inspired by [tbmk](https://github.com/linhx/tbmk).

## Usage

---

- search (get): `ctrl + g`
- delete: `ctrl + d`
- insert (bookmark): `ctrl + b`

The command-line buffer is copied into the search field (search mode) or the command field (insert mode).

## Install

---

1. Extract archive to its new home
2. cd into the directory and run `./install`
3. Restart shell or source config file

Bookmarks are saved into `./data/bookmarks.json`.

If you need to move the directory run `./install` again after moving.