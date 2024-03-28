use std::io::{Stdout, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{Terminal, TerminalOptions, Viewport};
use ratatui::backend::CrosstermBackend;

pub fn init_terminal(inline_height: u16) -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    // stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let options = TerminalOptions {
        viewport: Viewport::Inline(inline_height)
    };
    let terminal = Terminal::with_options(CrosstermBackend::new(stdout()), options)?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    // stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}