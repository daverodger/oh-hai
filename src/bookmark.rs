use nanoid::nanoid;
use ratatui::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    pub title: String,
    pub command: String,
    id: String,
}

impl Bookmark {
    pub fn new(title: String, command: String) -> Self {
        Bookmark {
            title,
            command,
            id: nanoid!(),
        }
    }

    pub fn tui_text(self) -> Text<'static> {
        Text::raw(format!("Title: {}\nCommand: {}", self.title, self.command))
    }
}

