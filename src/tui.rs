use std::io::{stdout, Stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::{Terminal, TerminalOptions, Viewport};

// Initializes inline terminal based on height param
pub fn init_terminal(
    inline_height: u16,
) -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let options = TerminalOptions {
        viewport: Viewport::Inline(inline_height),
    };
    let terminal = Terminal::with_options(CrosstermBackend::new(stdout()), options)?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    Ok(())
}
