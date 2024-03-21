use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::model::{AppState, Model};

pub fn view(frame: &mut Frame, model: &mut Model) {
    match model.app_state {
        AppState::Searching => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Min(2),
                ])
                .split(frame.size());

            frame.render_widget(
                model.free_text_area.widget(),
                layout[0],
            );

            frame.render_stateful_widget(
                Model::get_command_list(model.command_list.sorted_commands.clone()),
                layout[1],
                &mut model.command_list.state)
        }
        _ => ()
    }
}
