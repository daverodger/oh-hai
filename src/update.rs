use crossterm::event::KeyCode;
use tui_textarea::CursorMove;

use crate::bookmark::Bookmark;
use crate::matcher;
use crate::model::{Action, AppState, Model};

pub fn update(action: Action, model: &mut Model) {
    match model.app_state {
        AppState::Searching => searching_update(action, model),
        AppState::Inserting => inserting_update(action, model),
        AppState::Initializing => {
            match action {
                Action::Search => {
                    model.deserialize_commands();
                    model.reset_state();
                    model.app_state = AppState::Searching;
                }
                Action::Insert => {
                    model.app_state = AppState::Inserting;
                    // TODO get command line text and store somewhere
                }
                _ => ()
            }
        }
        _ => ()
    }
}

fn searching_update(action: Action, model: &mut Model) {
    match action {
        Action::KeyInput(key) => {
            model.search_text_area.input(key);
            let search_text = &model.search_text_area.lines()[0];
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
            // TODO output search_text_area somehow for use by bash script
            model.app_state = AppState::Done;
        }
        _ => ()
    }
}

fn inserting_update(action: Action, model: &mut Model) {
    match action {
        Action::KeyInput(key) => {
            model.insert_text_area.input(key);
            let insert_text = &model.insert_text_area.lines()[0];
            // TODO warn if duplicate name or command
            match key.code {
                KeyCode::Backspace => {
                    if insert_text.is_empty() {
                        // TODO display some warning must not be blank. needs to be for both lines though
                        // TODO Backspace must not delete the command line
                    }
                }
                _ => ()
            }
        }
        Action::EntryDown | Action::EntryUp => {
            match model.insert_text_area.cursor() {
                (0, _) => {
                    model.insert_text_area.move_cursor(CursorMove::Bottom);
                    model.insert_text_area.move_cursor(CursorMove::End);
                }
                (1, _) => {
                    model.insert_text_area.move_cursor(CursorMove::Top);
                    model.insert_text_area.move_cursor(CursorMove::End);
                }
                _ => ()
            }
        }
        Action::Exit => {
            model.app_state = AppState::Done;
        }
        Action::Submit => {
            let mut lines = model.insert_text_area.lines().to_owned();
            let (command, title) = (
                lines.pop().unwrap(),
                lines.pop().unwrap(),
            );
            let bm = Bookmark::new(title, command);
            let buffer = vec![bm];
            serde_yaml::to_writer(&model.bookmark_file, &buffer).unwrap();

            model.app_state = AppState::Done;
        }
        _ => ()
    }
}