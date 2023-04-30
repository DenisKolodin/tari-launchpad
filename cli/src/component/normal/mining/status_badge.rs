use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub struct StatusBadge {}

impl StatusBadge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for StatusBadge {
    fn on_input(&mut self, _key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for StatusBadge {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let text = "(running)";
        let style = Style::default().fg(Color::Green);
        let spans = Spans(vec![Span::styled(text, style)]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
