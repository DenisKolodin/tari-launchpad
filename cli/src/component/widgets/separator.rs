use crate::state::Focus;
use std::collections::HashSet;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols::line;
use tui::text::Span;
use tui::widgets::Widget;

pub struct Separator<'a> {
    focus_on: HashSet<Focus>,
    title: &'a str,
    line_set: line::Set,
}

impl<'a> Separator<'a> {
    pub fn new(title: &'a str) -> Self {
        Self::new_with_focus(title, [])
    }

    pub fn new_with_focus(title: &'a str, focus: impl IntoIterator<Item = Focus>) -> Self {
        Self {
            focus_on: focus.into_iter().collect(),
            title: title.into(),
            line_set: line::NORMAL,
        }
    }
}

impl<'a> Widget for Separator<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD);
        let span = Span::styled(self.title, style);
        let (col, row) = buf.set_span(area.left(), area.top(), &span, area.width);
        let start = col + 1;

        let y = row;
        for x in start..area.right() {
            buf.get_mut(x, y).set_symbol(self.line_set.horizontal); //"_"
        }
    }
}
