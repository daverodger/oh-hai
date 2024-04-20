use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{Color, Style};
use ratatui::Terminal;

use handle::handle;
use model::{Action, AppState, Model};

// TODO move all this shit to a lib.rs
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
                model.insert_text_area[1].insert_str(cmd);
            }
            model.insert_text_area[0].set_cursor_style(Style::default().bg(Color::White)); // TODO move to model?
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
