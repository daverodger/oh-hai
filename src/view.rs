use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, ListDirection, Paragraph};

use crate::model::{AppState, InsertState, Model};

pub const HIGHLIGHT_COLOR: Style = Style::new().fg(Color::LightGreen);
pub const COMMAND_PREFIX: char = '>';

pub fn view(frame: &mut Frame, model: &mut Model) {
    match &model.app_state {
        state @ AppState::Searching | state @ AppState::Deleting => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Max(1), Constraint::Min(2)])
                .split(frame.size());

            // Search text area style
            model.search_text_area.set_cursor_line_style(Style::default().white().on_black());
            model.search_text_area.set_cursor_style(Style::default().on_white().slow_blink());
            model.search_text_area.set_block(Block::default().white().on_black());

            if *state == AppState::Deleting {
                frame.render_widget(
                    "Delete selected entry? (y/n)".light_yellow().on_black(),
                    layout[0],
                );
            } else {
                frame.render_widget(
                    model.search_text_area.widget(),
                    layout[0],
                );
            }

            // Get and style command list
            let cmd_list = Model::get_fuzzied_cmd_list(model.command_list.sorted_commands.clone())
                .block(Block::default().title(format!("{} Command(s)", model.command_list.sorted_commands.len())).borders(Borders::ALL))
                .white()
                .on_black()
                .highlight_style(Style::default().bg(Color::DarkGray))
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(
                cmd_list,
                layout[1],
                &mut model.command_list.state,
            );
        }
        AppState::Inserting(insert_state) => {
            let mut block = Block::default()
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);

            match insert_state {
                InsertState::Unchecked => {
                    block = block
                        .title("New Command")
                        .white()
                        .on_black();
                }
                InsertState::Blank => {
                    block = block
                        .title("Save with blank fields? (y/n)")
                        .light_yellow()
                        .on_black();
                }
                InsertState::Duplicate => {
                    block = block
                        .title("Duplicate title or command exists. Save anyway? (y/n)")
                        .light_yellow()
                        .on_black();
                }
            }

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

            // Insert style
            model.insert_text_area[0].set_cursor_line_style(Style::default().fg(Color::White));
            model.insert_text_area[0].set_cursor_style(Style::default().fg(Color::White));
            model.insert_text_area[1].set_cursor_line_style(Style::default().fg(Color::White));
            model.insert_text_area[1].set_cursor_style(Style::default().fg(Color::White));
            // Show cursor only on focused line
            model.insert_text_area[model.focus_insert].set_cursor_style(Style::default().bg(Color::White));

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

