use nanoid::nanoid;
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

use crate::view::build_highlighted_text;

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

