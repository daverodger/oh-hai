use serde::{Deserialize, Serialize};
use nanoid::nanoid;
use ratatui::text::Text;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bookmark {
    pub title: String,
    pub command: String,
    id: String
}

impl Bookmark {
    pub(crate) fn new(title: String, command: String) -> Self {
        Bookmark {
            title,
            command,
            id: nanoid!()
        }
    }

    pub fn tui_text(self) -> Text<'static> {
        Text::raw(format!("Title: {}\nCommand: {}", self.title, self.command))

    }
}
