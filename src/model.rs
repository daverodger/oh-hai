use std::fs::File;

use crossterm::event::KeyEvent;
use ratatui::prelude::{Color, Style, Stylize, Text};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use tui_textarea::TextArea;

use crate::bookmark::Bookmark;

#[derive(Debug, Clone)]
pub struct Model<'a> {
    pub app_state: AppState,
    pub command_list: StatefulList,
    pub free_text_area: TextArea<'a>,
}

#[derive(Debug, Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub commands: Vec<Bookmark>,
    pub sorted_commands: Vec<Bookmark>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
            command_list: StatefulList {
                state: ListState::default(),
                commands: Vec::new(),
                sorted_commands: Vec::new(),
            },
            free_text_area: styled_text_area(),
        }
    }

    pub fn deserialize_commands(&mut self) {
        let bookmark_file = Self::get_bookmark_file().expect("File should exist or have been created");
        self.command_list.commands = serde_yaml::from_reader(bookmark_file).unwrap_or(vec![]);
        self.command_list.sorted_commands = self.command_list.commands.clone();
    }

    pub fn reset_state(&mut self) {
        self.command_list.state.select(Some(0));
    }

    pub fn get_command_list(bookmarks: Vec<Bookmark>) -> List<'static> {
        List::new(bookmarks.into_iter().map(|x| x.tui_text()).collect::<Vec<Text>>())
            .block(Block::default().title("Saved Commands").borders(Borders::ALL))
            .white()
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol(">>")
            .direction(ListDirection::TopToBottom)
    }

    fn get_bookmark_file() -> Result<File, Box<dyn std::error::Error>> {
        let bookmark_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(BOOKMARK_FILE)?;
        Ok(bookmark_file)
    }

    pub fn sorted_command_len(&self) -> usize {
        self.command_list.sorted_commands.len()
    }
}

fn styled_text_area() -> TextArea<'static> {
    let mut ta = TextArea::default();

    let line_style = Style::default().fg(Color::White);
    ta.set_cursor_line_style(line_style);

    let cursor_style = Style::default().bg(Color::White).slow_blink();
    ta.set_cursor_style(cursor_style);

    ta
}