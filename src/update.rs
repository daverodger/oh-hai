use crossterm::event;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use tui_textarea::TextArea;
use crate::bookmark::Bookmark;
use crate::{Action, AppState, Model};

pub(crate) fn update(action: Action, model: &mut Model) {
    match action {
        Action::Search => {
            deserialize_commands(model);
            model.app_state = AppState::Searching;
            model.highlighted_command = 0;
        }
        Action::KeyInput(key) => {
            match key.code {
                event::KeyCode::Char(_) | event::KeyCode::Backspace => {
                    model.free_text_area.input(key);
                }
                event::KeyCode::Enter => {
                    print!("{}", model.free_text_area.yank_text()); // TODO does this work to output to bash variable?
                }
                _ => todo!()
            }
        }
        _ => {}
    }
}
pub(crate) fn deserialize_commands(model: &mut Model) {
    let deserialized_bookmarks: Vec<Bookmark> = serde_yaml::from_reader(&model.bookmark_file).unwrap_or(vec![]);
    model.commands = Some(List::new(
        deserialized_bookmarks.into_iter()
            .map(|b| b.tui_text())
            .collect::<Vec<Text>>())
        .block(Block::default().title("Saved Commands").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">>")
        .direction(ListDirection::TopToBottom));
}
