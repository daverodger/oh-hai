use std::io::{stdout, prelude::*, BufReader};
use std::fs::File;
use std::string::ToString;
use crossterm::{event::{self, KeyEventKind}, ExecutableCommand, terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
}};
use ratatui::{
    prelude::*,
};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use tui_textarea::TextArea;
use bookmark::Bookmark;

mod bookmark;

struct Model {
    app_state: AppState,
    highlighted_command: usize,
    bookmark_file: File,
}

enum AppState {
    Searching,
    Inserting,
    Initializing,
}

enum Actions {
    Delete,
    Insert,
    EntryDown,
    EntryUp,
    ReturnCommand,
}

const BOOKMARK_FILE: String = "bookmarks.yaml".to_string(); // TODO use const fn to read config name/location?

fn main() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut bookmark_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(BOOKMARK_FILE)?;

    let model = Model {
        app_state: AppState::Initializing,
        highlighted_command: 0,
        bookmark_file,
    };

    let deserialized_bookmarks: Vec<Bookmark> = serde_yaml::from_reader(File::open("bookmarks.yaml")?)?;
    let mut state = ListState::default();
    state.select(Some(0)); // TODO manipulate this inside event loop
    let command_list = List::new(
        deserialized_bookmarks.iter()
            .map(|b| b.tui_text())
            .collect::<Vec<Text>>())
        .block(Block::default().title("Saved Commands").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom);


    let mut text_area = TextArea::default();
    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Min(2),
                ])
                .split(frame.size());

            frame.render_widget(
                text_area.widget(),
                layout[0],
            );

            frame.render_stateful_widget(
                &command_list,
                layout[1],
                &mut state)
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
                            print!("{}", text_area.yank_text()); // TODO does this work?
                            break;
                        }
                        _ => todo!()
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
