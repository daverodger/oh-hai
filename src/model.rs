use std::fs::File;

use crossterm::event::KeyEvent;
use ratatui::prelude::Text;
use ratatui::widgets::{List, ListState};
use tui_textarea::TextArea;

use crate::bookmark::Bookmark;
use crate::config;

#[derive(Debug)]
pub struct Model<'a> {
    pub app_state: AppState,
    pub command_list: StatefulList,
    pub search_text_area: TextArea<'a>,
    pub insert_text_area: [TextArea<'a>; 2],
    pub focus_insert: usize,
    pub bookmark_file: File,
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
    Inserting(InsertState),
    Done,
    Initializing,
    Deleting,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InsertState {
    Unchecked,
    Blank,
    Duplicate,
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

impl Model<'_> {
    pub fn new() -> Self {
        let bookmark_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(config::get_data_file_path().as_path()).unwrap();

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
            bookmark_file,
        }
    }

    pub fn deserialize_commands(&mut self) {
        self.command_list.commands = serde_json::from_reader(&self.bookmark_file).expect("unable to parse bookmarks.json"); // TODO breaks shit
        self.command_list.sorted_commands = self.command_list.commands.clone();
    }

    pub fn reset_state(&mut self) {
        self.command_list.state.select(Some(0));
    }

    pub fn get_fuzzied_cmd_list(bookmarks: Vec<Bookmark>) -> List<'static> {
        List::new(bookmarks.into_iter().map(|x| x.tui_text_fuzzy()).collect::<Vec<Text>>())
    }

    pub fn sorted_command_len(&self) -> usize {
        self.command_list.sorted_commands.len()
    }

    pub fn get_selected_index(&self) -> usize {
        self.command_list.state.selected().unwrap()
    }
}