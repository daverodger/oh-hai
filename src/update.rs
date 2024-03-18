use crossterm::event;

use crate::model::{Action, AppState, Model};

pub fn update(action: Action, model: &mut Model) {
    match action {
        Action::Search => {
            model.deserialize_commands();
            model.app_state = AppState::Searching;
            model.active_command = 0;
        }
        Action::KeyInput(key) => {
            match key.code {
                event::KeyCode::Char(_) | event::KeyCode::Backspace => {
                    model.free_text_area.input(key);
                }
                _ => todo!()
            }
        }
        Action::EntryDown => {
            model.active_command += 1;
            if model.active_command >= model.command_list_len() {
                model.active_command = 0;
            }
        }
        Action::EntryUp => {
            if model.active_command == 0 {
                model.active_command = model.command_list_len() - 1;
            } else {
                model.active_command -= 1;
            }
        }
        Action::Exit => {
            model.app_state = AppState::Done;
        }
        Action::Submit => {
            print!("{}", model.free_text_area.yank_text()); // TODO does this work to output to bash variable?
            model.app_state = AppState::Done;
        }
        _ => {}
    }
}
