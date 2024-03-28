use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tui_textarea::CursorMove;

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

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut model = Model::new();

    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.get(0).expect("No flag provided (-i or -s)").as_ref() {
        "-s" => {
            terminal = tui::init_terminal(9)?;
            update::update(Action::Search, &mut model);
        },
        "-i" => {
            terminal = tui::init_terminal(4)?;
            if let Some(cmd) = args.get(1) {
                model.insert_text_area.move_cursor(CursorMove::Down);
                model.insert_text_area.insert_str(cmd);
                model.insert_text_area.move_cursor(CursorMove::Up);
            }
            update::update(Action::Insert, &mut model);
        },
        _ => ()
    }

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
