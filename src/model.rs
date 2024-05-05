use std::fs::File;

use crossterm::event::KeyEvent;
use ratatui::prelude::{Color, Style, Stylize, Text};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use tui_textarea::TextArea;

use crate::bookmark::Bookmark;

#[derive(Debug)]
pub struct Model<'a> {
    pub app_state: AppState,
    pub command_list: StatefulList,
    pub search_text_area: TextArea<'a>,
    pub insert_text_area: [TextArea<'a>; 2],
    pub focus_insert: usize,
    pub bookmark_file: File
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
    Initializing,
    Deleting,
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

const BOOKMARK_FILE: &'static str = "bookmarks.json"; // TODO use const fn to read config name/location?

impl Model<'_> {
    pub fn new() -> Self {
        Model {
            app_state: AppState::Initializing,
            command_list: StatefulList {
                state: ListState::default(),
                commands: Vec::new(),
                sorted_commands: Vec::new(),
            },
            search_text_area: TextArea::default(),
            insert_text_area: [TextArea::default(), TextArea::default()],
            focus_insert: 0,
            bookmark_file: Self::get_bookmark_file().expect("File should exist")
        }
    }

    pub fn deserialize_commands(&mut self) {
        self.command_list.commands = serde_json::from_reader(&self.bookmark_file).unwrap_or(vec![]);
        self.command_list.sorted_commands = self.command_list.commands.clone();
    }

    pub fn reset_state(&mut self) {
        self.command_list.state.select(Some(0));
    }

    pub fn get_command_list(bookmarks: Vec<Bookmark>) -> List<'static> {
        let len = bookmarks.len();
        List::new(bookmarks.into_iter().map(|x| x.tui_text_fuzzy()).collect::<Vec<Text>>())
            .block(Block::default().title(format!("{} Command(s)", len)).borders(Borders::ALL))
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

    pub fn get_selected_index(&self) -> usize {
        self.command_list.state.selected().unwrap()
    }
}
