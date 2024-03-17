use std::io::prelude::*;
use std::string::ToString;

use crossterm::ExecutableCommand;
use ratatui::prelude::*;

use model::{AppState, Model};

use crate::handle::handle;

mod bookmark;
mod view;
mod update;
mod handle;
mod tui;
mod model;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut terminal = tui::init_terminal()?;

    let mut model = Model::new();

    model.deserialize_commands(); // TODO only do this if in search mode... update does this too?

    while model.app_state != AppState::Done {
        terminal.draw(|frame| {
            view::view(frame, &model);
        })?;

        let message = handle();

        if let Some(action) = message {
            update::update(action, &mut model);
        }
    }

    tui::restore_terminal()
}
