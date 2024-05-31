use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use handle::handle;
use model::*;
use oh_hai::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut model = Model::new();

    // Init terminal
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.get(0).expect("No flag provided (-i or -s)").as_ref() {
        "-s" => {
            // Start in search mode
            terminal = tui::init_terminal(9)?;
            update::update(Action::Search, &mut model);
        }
        "-i" => {
            // Start in insert mode
            terminal = tui::init_terminal(4)?;

            // Populate new command field with remaining args
            let mut new_command = String::new();
            let mut i = 1;
            while let Some(cmd) = args.get(i) {
                if i > 1 {
                    new_command.push_str(" ");
                }
                new_command.push_str(cmd);
                i += 1;
            }
            model.insert_text_area[1].insert_str(new_command);

            update::update(Action::Insert, &mut model);
        }
        _ => (),
    }

    // Main program loop
    while model.app_state != AppState::Done {
        terminal.draw(|frame| {
            view::view(frame, &mut model);
        })?;

        let message = handle();

        if let Some(action) = message {
            update::update(action, &mut model);
        }
    }

    // Restore terminal
    terminal.clear()?;
    tui::restore_terminal()
}
