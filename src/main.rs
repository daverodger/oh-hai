use std::io::prelude::*;
use std::fs::File;
use crossterm::{event::{self, KeyEventKind}, ExecutableCommand, QueueableCommand, terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
}};
use ratatui::{
    prelude::*,
};
use tui_textarea::TextArea;
use std::io::stdout;
use bookmark::Bookmark;

mod bookmark;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let bmk = Bookmark {
        title: "Test command".to_string(),
        command: "grep | deez nuts".to_string(),
    };
    let yaml = serde_yaml::to_string(&bmk)?;
    let mut file = File::create("bookmarks.yaml")?;
    file.write_all(yaml.as_bytes())?;

    let mut text_area = TextArea::default();
    loop {
        terminal.draw(|frame| {
            let search_area = frame.size();
            frame.render_widget(
                text_area.widget(), search_area
            )
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char(_) | event::KeyCode::Backspace => {
                            text_area.input(key);
                            continue;
                        }
                        event::KeyCode::Enter => {
                            print!("{}", text_area.yank_text());
                            break;
                        }
                        _ => todo!()
                    }
                }
            }
        }
    }

    // TODO output command upon exit

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

// TODO use tui_widget_list for search list maybe
