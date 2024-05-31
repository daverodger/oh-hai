use nanoid::nanoid;
use ratatui::prelude::{Line, Span};
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    id: String,
    pub title: String,
    pub command: String,
    #[serde(skip)]
    pub title_highlights: Vec<usize>,
    #[serde(skip)]
    pub command_highlights: Vec<usize>,
}

impl Bookmark {
    pub fn new(title: &str, command: &str) -> Self {
        let title = title.trim().to_string();
        let command = command.trim().to_string();
        Bookmark {
            title,
            command,
            title_highlights: vec![],
            command_highlights: vec![],
            id: nanoid!(),
        }
    }

    pub fn tui_text_fuzzy(self) -> Text<'static> {
        let mut text = Text::from(vec![
            build_highlighted_text(self.title, self.title_highlights),
            build_highlighted_text(self.command, self.command_highlights),
        ]);

        // add command prefix symbol
        text.lines
            .get_mut(1)
            .expect("Second line should exist since we just built it")
            .spans
            .insert(0, Span::from(crate::view::COMMAND_PREFIX.to_string()));
        text
    }
}

impl PartialEq for Bookmark {
    fn eq(&self, other: &Self) -> bool {
        if other.title == self.title || other.command == self.command {
            return true;
        }
        false
    }
}

pub fn build_highlighted_text(s: String, arr: Vec<usize>) -> Line<'static> {
    let mut line = Vec::new();
    let mut arr = arr;
    for c in s.char_indices() {
        if !arr.is_empty() && arr[0] == c.0 {
            arr.remove(0);
            line.push(Span::styled(c.1.to_string(), crate::view::HIGHLIGHT_COLOR));
        } else {
            line.push(Span::raw(c.1.to_string()));
        }
    }
    Line::from(line)
}
