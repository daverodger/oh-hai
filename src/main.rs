use handle::handle;
use model::{Action, AppState, Model};

mod bookmark;
mod view;
mod update;
mod handle;
mod tui;
mod model;
mod matcher;


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
