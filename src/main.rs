use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use handle::handle;
use model::*;
use oh_hai::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let log_file = File::create("debug.log")?;
    // let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(log_file)).pretty();
    // tracing_subscriber::registry().with(debug_log.with_filter(filter::LevelFilter::DEBUG)).init();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut model = Model::new();

    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.get(0).expect("No flag provided (-i or -s)").as_ref() {
        "-s" => {
            terminal = tui::init_terminal(9)?;
            update::update(Action::Search, &mut model);
        }
        "-i" => {
            terminal = tui::init_terminal(4)?;
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
