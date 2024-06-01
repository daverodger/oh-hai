use std::hash::{Hash, Hasher};

use nanoid::nanoid;
use ratatui::prelude::{Line, Span};
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    pub id: String,
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

    // Returns fuzzy highlighted Text struct containing title and command
    pub fn tui_text_fuzzy(self) -> Text<'static> {
        let mut text = Text::from(vec![
            build_highlighted_text(self.title, self.title_highlights),
            build_highlighted_text(self.command, self.command_highlights),
        ]);

        // Add command prefix symbol
        text.lines
            .get_mut(1)
            .expect("Second line should exist since we just built it")
            .spans
            .insert(0, Span::from(crate::view::COMMAND_PREFIX.to_string()));
        text
    }
}

// Ignores highlights fields
impl PartialEq for Bookmark {
    fn eq(&self, other: &Self) -> bool {
        if other.title == self.title && other.command == self.command && other.id == self.id {
            return true;
        }
        false
    }
}

// Cannot derive as highlights fields not relevant
impl Eq for Bookmark {}

// Cannot derive as highlights fields not relevant
impl Hash for Bookmark {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.command.hash(state);
        self.title.hash(state);
    }
}

// Reconstructs input String as Line from highlighted Spans based on input array indexes
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
