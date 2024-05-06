use nanoid::nanoid;
use ratatui::prelude::{Line, Span};
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    pub title: String,
    pub command: String,
    #[serde(skip)]
    pub title_highlights: Vec<usize>,
    #[serde(skip)]
    pub command_highlights: Vec<usize>,
    id: String,
}

impl Bookmark {
    pub fn new(title: String, command: String) -> Self {
        Bookmark {
            title,
            command,
            title_highlights: vec![],
            command_highlights: vec![],
            id: nanoid!(),
        }
    }

    pub fn tui_text_fuzzy(self) -> Text<'static> {
        Text::from(
            vec!(build_highlighted_text(self.title, self.title_highlights),
                 build_highlighted_text(self.command, self.command_highlights)
            )
        )
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
