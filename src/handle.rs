use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, poll};

use crate::model::Action;

pub fn handle() -> Option<Action> {
    if poll(std::time::Duration::from_millis(16)).ok()? {
        if let Event::Key(key) = event::read().ok()? {
            if key.kind == KeyEventKind::Press {
                return match (key.code, key.modifiers) {
                    (KeyCode::Down, _) | (KeyCode::Tab, _) => Some(Action::EntryDown),
                    (KeyCode::Up, _) | (KeyCode::BackTab, _) => Some(Action::EntryUp),
                    (KeyCode::Esc, _) => Some(Action::Exit),
                    (KeyCode::Enter, _) => Some(Action::Submit),
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => return Some(Action::Delete),
                    _ => Some(Action::KeyInput(key))
                };
            }
        }
    }
    None
}