use model::{AppState, Model};

use crate::handle::handle;
use crate::model::Action;

mod bookmark;
mod view;
mod update;
mod handle;
mod tui;
mod model;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut terminal = tui::init_terminal()?;

    let mut model = Model::new();

    update::update(Action::Search, &mut model);

    while model.app_state != AppState::Done {
        terminal.draw(|frame| {
            view::view(frame, &mut model);
        })?;

        let message = handle();

        if let Some(action) = message {
            update::update(action, &mut model);
        }
    }

    terminal.clear()?;
    tui::restore_terminal()
}
