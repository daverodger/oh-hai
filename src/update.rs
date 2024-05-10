use std::fs::File;
use std::io::Write;

use crossterm::event::KeyCode;

use crate::bookmark::Bookmark;
use crate::matcher;
use crate::model::{Action, AppState, InsertState, Model};

pub fn update(action: Action, model: &mut Model) {
    match model.app_state {
        AppState::Searching => searching_update(action, model),
        AppState::Inserting(InsertState::Unchecked) => inserting_update(action, model),
        AppState::Inserting(_)=> confirm_insert(action, model),
        AppState::Deleting => delete_popup_update(action, model),
        AppState::Initializing => {
            match action {
                Action::Search => {
                    model.deserialize_commands();
                    model.reset_state();
                    model.app_state = AppState::Searching;
                }
                Action::Insert => {
                    model.deserialize_commands();
                    model.app_state = AppState::Inserting(InsertState::Unchecked);
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
            let _ = File::create(".command.txt");
            model.app_state = AppState::Done;
        }
        Action::Submit => {
            let mut file = File::create(".command.txt").unwrap();
            let selected_command = &model.command_list.sorted_commands.get(model.get_selected_index()).unwrap().command;
            file.write(selected_command.as_bytes()).unwrap();

            model.app_state = AppState::Done;
        }
        Action::Delete => {
            model.app_state = AppState::Deleting;
        }
        _ => ()
    }
}

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
            if model.insert_text_area[0].is_empty() || model.insert_text_area[1].is_empty() {
                model.app_state = AppState::Inserting(InsertState::Blank);
            } else if model.command_list.commands.contains(&create_bookmark_from_model(model)){
                model.app_state = AppState::Inserting(InsertState::Duplicate);
            } else {
                insert_bookmark(model);
            }
        }
        _ => ()
    }
}

fn delete_popup_update(action: Action, model: &mut Model) {
    if let Action::KeyInput(key) = action {
        if key.code == KeyCode::Char('y') || key.code == KeyCode::Char('Y') {
            let _ = &model.command_list.sorted_commands.remove(model.get_selected_index());
            let _ = &model.command_list.commands.remove(model.get_selected_index());
            model.bookmark_file = File::create("bookmarks.json").unwrap();
            serde_json::to_writer_pretty(&model.bookmark_file, &model.command_list.commands).unwrap();
        }
    }
    model.app_state = AppState::Searching;
}

fn confirm_insert(action: Action, model: &mut Model) {
    if let Action::KeyInput(key) = action {
        if key.code == KeyCode::Char('y') || key.code == KeyCode::Char('Y') {
            insert_bookmark(model);
            return
        }
    }
    model.app_state = AppState::Inserting(InsertState::Unchecked);
}

fn insert_bookmark(model: &mut Model) {
    let bm = create_bookmark_from_model(model);
    model.command_list.commands.push(bm);
    model.bookmark_file = File::create("bookmarks.json").unwrap();
    serde_json::to_writer_pretty(&model.bookmark_file, &model.command_list.commands).unwrap();
    model.app_state = AppState::Done;
}

fn create_bookmark_from_model(model: &Model) -> Bookmark {
    let title = model.insert_text_area[0].lines()[0].to_string();
    let command = model.insert_text_area[1].lines()[0].to_string();
    Bookmark::new(title, command)
}