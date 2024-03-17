use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::ListState;
use tui_textarea::TextArea;
use crate::{AppState, Model};

pub(crate) fn view(frame: &mut Frame, model: &Model) {
    match model.app_state {
        AppState::Searching => {
            let mut state = ListState::default();
            state.select(Some(model.highlighted_command)); // TODO manipulate this inside event loop
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

            if let Some(_) = &model.commands {
                frame.render_stateful_widget(
                    model.commands.clone().expect("Check already confirmed not None variant"),
                    layout[1],
                    &mut state)
            }
        }
        _ => ()
    }
}
