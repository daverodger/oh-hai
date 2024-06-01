use std::fs::File;
use std::io::Write;

use crossterm::event::KeyCode;

use crate::bookmark::Bookmark;
use crate::model::{Action, AppState, InsertState, Model};
use crate::{config, matcher};

pub fn update(action: Action, model: &mut Model) {
    match model.app_state {
        AppState::Searching => searching_update(action, model),
        AppState::Inserting(InsertState::Unchecked) => inserting_update(action, model),
        AppState::Inserting(_) => confirm_insert(action, model),
        AppState::Deleting => delete_popup_update(action, model),
        AppState::Initializing => match action {
            Action::Search => {
                model.deserialize_commands();
                model.reset_state();
                model.app_state = AppState::Searching
            }
            Action::Insert => {
                model.deserialize_commands();
                model.app_state = AppState::Inserting(InsertState::Unchecked);
            }
            _ => (),
        },
        _ => (),
    }
}

// Update logic for search mode
fn searching_update(action: Action, model: &mut Model) {
    match action {
        Action::KeyInput(key) => {
            model.search_text_area.input(key);
            let search_text = &model.search_text_area.lines()[0];
            match key.code {
                KeyCode::Backspace => {
                    if search_text.is_empty() {
                        model.reset_sorted();
                    } else {
                        model.command_list.sorted_commands =
                            matcher::sort(model.command_list.commands.clone(), search_text);
                    }
                }
                _ => {
                    model.command_list.sorted_commands =
                        matcher::sort(model.command_list.sorted_commands.clone(), search_text);
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
            // Needed to truncate the output file so the last command isn't re-exported
            let _ = File::create(config::get_output_file_path());

            model.app_state = AppState::Done;
        }
        Action::Submit => {
            // Writes selected command to file which is copied to Readline line buffer by shell/key-bindings.bash
            let mut file = File::create(config::get_output_file_path()).unwrap();
            let selected_command = &model
                .command_list
                .sorted_commands
                .get(model.get_selected_index())
                .unwrap()
                .command;
            file.write(selected_command.as_bytes()).unwrap();

            model.app_state = AppState::Done;
        }
        Action::Delete => {
            // Only delete if visible command list isn't empty
            if model.command_list.sorted_commands.len() > 0 {
                model.app_state = AppState::Deleting;
            }
        }
        _ => (),
    }
}

// Update logic for insert mode
fn inserting_update(action: Action, model: &mut Model) {
    match action {
        Action::KeyInput(key) => {
            model.insert_text_area[model.focus_insert].input(key);
        }
        Action::EntryDown | Action::EntryUp => {
            model.focus_insert = (model.focus_insert + 1) % 2;
        }
        Action::Exit => {
            model.app_state = AppState::Done;
        }
        Action::Submit => {
            // Check blank fields
            if model.insert_text_area[0].is_empty() || model.insert_text_area[1].is_empty() {
                model.app_state = AppState::Inserting(InsertState::Blank);
            }
            // Check duplicates
            else if model
                .command_list
                .commands
                .contains(&create_bookmark_from_model(model))
            {
                model.app_state = AppState::Inserting(InsertState::Duplicate);
            } else {
                insert_bookmark(model);
            }
        }
        _ => (),
    }
}

// Update logic for delete confirmation view
fn delete_popup_update(action: Action, model: &mut Model) {
    if let Action::KeyInput(key) = action {
        if key.code == KeyCode::Char('y') || key.code == KeyCode::Char('Y') {
            // Remove command from primary command list
            model.remove_selected_command();

            // Rewrite bookmark file without deleted command
            model.bookmark_file = File::create(config::get_data_file_path().as_path()).unwrap();
            serde_json::to_writer_pretty(&model.bookmark_file, &model.command_list.commands)
                .unwrap();
            model.search_text_area.delete_line_by_head();
            model.search_text_area.delete_line_by_end();
            model.reset_sorted();
            model.reset_state();
        }
        model.app_state = AppState::Searching;
    }
}

// Update logic used during an insertion confirmation
fn confirm_insert(action: Action, model: &mut Model) {
    if let Action::KeyInput(key) = action {
        if key.code == KeyCode::Char('y') || key.code == KeyCode::Char('Y') {
            insert_bookmark(model);
            return;
        }
    }
    model.app_state = AppState::Inserting(InsertState::Unchecked);
}

// Generates Bookmark from contents of insert_text_area and updates data file
fn insert_bookmark(model: &mut Model) {
    let bm = create_bookmark_from_model(model);
    model.command_list.commands.insert(bm);
    model.bookmark_file = File::create(config::get_data_file_path().as_path()).unwrap();
    serde_json::to_writer_pretty(&model.bookmark_file, &model.command_list.commands).unwrap();
    model.app_state = AppState::Done;
}

fn create_bookmark_from_model(model: &Model) -> Bookmark {
    let title = model.insert_text_area[0].lines()[0].as_str();
    let command = model.insert_text_area[1].lines()[0].as_str();
    Bookmark::new(title, command)
}
