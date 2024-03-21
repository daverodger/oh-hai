use crossterm::event::KeyCode;

use crate::matcher;
use crate::model::{Action, AppState, Model};

pub fn update(action: Action, model: &mut Model) {
    match action {
        Action::Search => {
            model.deserialize_commands();
            model.reset_state();
            model.app_state = AppState::Searching;
        }
        Action::KeyInput(key) => {
            model.free_text_area.input(key);
            let search_text = &model.free_text_area.lines()[0];
            match key.code {
                KeyCode::Backspace => {
                    if search_text.is_empty() {
                        model.command_list.sorted_commands = model.command_list.commands.clone();
                    } else {
                        model.command_list.sorted_commands = matcher::sort(model.command_list.commands.clone(), search_text);
                    }
                }
                _ => {
                    model.command_list.sorted_commands = matcher::sort(model.command_list.sorted_commands.clone(), search_text);
                }
            }
            model.reset_state();
        }
        Action::EntryDown => {
            let len = model.sorted_command_len();
            model.command_list.state.selected_mut().as_mut().map(|x| {
                if *x + 1 >= len {
                    *x = 0;
                } else {
                    *x += 1;
                }
            });
        }
        Action::EntryUp => {
            let len = model.sorted_command_len();
            model.command_list.state.selected_mut().as_mut().map(|x| {
                if *x == 0 {
                    *x = len - 1;
                } else {
                    *x -= 1;
                }
            });
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
