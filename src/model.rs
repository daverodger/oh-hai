use std::fs::File;

use crossterm::event::KeyEvent;
use ratatui::prelude::{Color, Style, Stylize, Text};
use ratatui::widgets::{Block, Borders, List, ListDirection};
use tui_textarea::TextArea;

use crate::bookmark::Bookmark;

#[derive(Debug)]
pub struct Model<'a> {
    pub app_state: AppState,
    pub active_command: usize,
    pub bookmark_file: File,
    pub commands: Option<List<'a>>,
    pub free_text_area: TextArea<'a>
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppState {
    Searching,
    Inserting,
    Done,
}

#[derive(PartialEq)]
pub enum Action {
    Delete,
    Insert,
    EntryDown,
    EntryUp,
    ReturnCommand,
    Search,
    KeyInput(KeyEvent),
    Exit,
    Submit,
}

const BOOKMARK_FILE: &'static str = "bookmarks.yaml"; // TODO use const fn to read config name/location?

impl Model<'_> {
    pub fn new() -> Self {
        Model {
            app_state: AppState::Searching, // TODO should be set based on program entry
            active_command: 0,
            bookmark_file: Self::get_bookmark_file().expect("File should either already exist or have been created"),
            commands: None,
            free_text_area: styled_text_area()
        }
    }

    pub fn deserialize_commands(&mut self) {
        let deserialized_bookmarks: Vec<Bookmark> = serde_yaml::from_reader(&self.bookmark_file).unwrap_or(vec![]);
        self.commands = Some(List::new(
            deserialized_bookmarks.into_iter()
                .map(|b| b.tui_text())
                .collect::<Vec<Text>>())
            .block(Block::default().title("Saved Commands").borders(Borders::ALL))
            .white()
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol(">>")
            .direction(ListDirection::TopToBottom));
    }

    fn get_bookmark_file() -> Result<File, Box<dyn std::error::Error>> {
        let bookmark_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(BOOKMARK_FILE)?;
        Ok(bookmark_file)
    }

    pub fn command_list_len(&self) -> usize {
        self.commands.as_ref().map_or(0, |x| x.len())
    }
}

fn styled_text_area() -> TextArea<'static> {
    let mut ta = TextArea::default();

    let line_style = Style::default().fg(Color::White);
    ta.set_cursor_line_style(line_style);

    let cursor_style =Style::default().bg(Color::White).slow_blink();
    ta.set_cursor_style(cursor_style);

    ta
}