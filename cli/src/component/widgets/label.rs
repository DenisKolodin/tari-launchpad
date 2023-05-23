use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::Widget;

pub struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().add_modifier(Modifier::BOLD);
        let span = Span::styled(self.text, style);
        buf.set_span(area.left(), area.top(), &span, area.width);
    }
}
