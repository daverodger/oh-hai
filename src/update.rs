use crossterm::event;

use crate::model::{Action, AppState, Model};

pub fn update(action: Action, model: &mut Model) {
    match action {
        Action::Search => {
            model.deserialize_commands();
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
                    model.app_state = AppState::Done;
                }
                _ => todo!()
            }
        }
        _ => {}
    }
}
