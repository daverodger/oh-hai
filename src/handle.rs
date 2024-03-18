use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};

use crate::model::Action;

pub fn handle() -> Option<Action> {
    if event::poll(std::time::Duration::from_millis(16)).ok()? {
        if let event::Event::Key(key) = event::read().ok()? {
            if key.kind == KeyEventKind::Press {
                return match key.code {
                    KeyCode::Down => Some(Action::EntryDown),
                    KeyCode::Up => Some(Action::EntryUp),
                    KeyCode::Esc => Some(Action::Exit),
                    KeyCode::Enter => Some(Action::Submit),
                    _ => Some(Action::KeyInput(key))
                }
            }
        }
    }
    None
}