use std::io::{Stdout, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{Terminal, TerminalOptions, Viewport};
use ratatui::backend::CrosstermBackend;

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>>{
    // stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let options = TerminalOptions {
        viewport: Viewport::Inline(7)
    };
    let terminal = Terminal::with_options(CrosstermBackend::new(stdout()), options)?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<(), Box<dyn std::error::Error>>{
    // stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}