use crossterm::event;
use crossterm::event::KeyEventKind;

use crate::model::Action;

pub fn handle() -> Option<Action> {
    if event::poll(std::time::Duration::from_millis(16)).ok()? {
        if let event::Event::Key(key) = event::read().ok()? {
            if key.kind == KeyEventKind::Press {
                return Some(Action::KeyInput(key))
            }
        }
    }
    None
}