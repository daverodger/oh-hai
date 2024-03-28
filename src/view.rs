use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::model::{AppState, Model};

pub fn view(frame: &mut Frame, model: &mut Model) {
    match model.app_state {
        AppState::Searching => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Max(1), Constraint::Min(2)])
                .split(frame.size());

            frame.render_widget(
                model.search_text_area.widget(),
                layout[0],
            );

            frame.render_stateful_widget(
                Model::get_command_list(model.command_list.sorted_commands.clone()),
                layout[1],
                &mut model.command_list.state,
            );
        }
        AppState::Inserting => {
            let block = Block::default()
                .title("New Command")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);

            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Max(1),
                    Constraint::Max(1),
                    Constraint::Max(1),
                ])
                .split(frame.size());

            let title_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Max(10),
                    Constraint::Min(1),
                ])
                .split(outer_layout[1]);

            let cmd_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Max(10),
                    Constraint::Min(1),
                ])
                .split(outer_layout[2]);

            frame.render_widget(
                block,
                frame.size(),
            );

            frame.render_widget(
                Paragraph::new(Line::raw("Name: ").right_aligned()),
                title_row[0],
            );

            frame.render_widget(
                model.insert_text_area[0].widget(),
                title_row[1],
            );

            frame.render_widget(
                Paragraph::new(Line::raw("Command: ").right_aligned()),
                cmd_row[0],
            );

            frame.render_widget(
                model.insert_text_area[1].widget(),
                cmd_row[1],
            );
        }
        _ => ()
    }
}
