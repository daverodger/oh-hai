use std::io::{stdout, prelude::*, BufReader};
use std::fs::File;
use std::string::ToString;
use crossterm::{event::{self, KeyEventKind}, ExecutableCommand, terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
}};
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use tui_textarea::TextArea;
use bookmark::Bookmark;
use crate::handle::handle;
use crate::update::{deserialize_commands, update};

mod bookmark;
mod view;
mod update;
mod handle;

#[derive(Debug)]
struct Model<'a> {
    app_state: AppState,
    highlighted_command: usize,
    bookmark_file: File,
    commands: Option<List<'a>>,
    free_text_area: TextArea<'a>
}

#[derive(Debug, PartialEq, Eq)]
enum AppState {
    Searching,
    Inserting,
    Done,
}

#[derive(PartialEq)]
enum Action {
    Delete,
    Insert,
    EntryDown,
    EntryUp,
    ReturnCommand,
    Search,
    KeyInput(KeyEvent),
}

const BOOKMARK_FILE: &'static str = "bookmarks.yaml"; // TODO use const fn to read config name/location?

fn main() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut bookmark_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(BOOKMARK_FILE)?;

    let mut model = Model {
        app_state: AppState::Searching, // TODO should be set based on program entry
        highlighted_command: 0,
        bookmark_file,
        commands: None,
        free_text_area: TextArea::default()
    };
    deserialize_commands(&mut model); // TODO only do this if searching

    while model.app_state != AppState::Done {
        terminal.draw(|frame| {
            view::view(frame, &model);
        })?;

        let message = handle();

        if let Some(action) = message {
            update::update(action, &mut model);
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
