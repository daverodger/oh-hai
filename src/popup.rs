use derive_setters::Setters;
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    pub title: Line<'a>,
    #[setters(into)]
    pub content: Text<'a>,
    pub border_style: Style,
    pub title_style: Style,
    pub style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}