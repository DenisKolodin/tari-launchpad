use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use std::marker::PhantomData;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub trait StatusGetter {
    fn get_status(&self, state: &LaunchpadState) -> (&str, Color);
}

pub struct StatusBadge<G> {
    getter: G,
}

impl<G> StatusBadge<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for StatusBadge<G> {
    fn on_input(&mut self, _key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend, G> Component<B> for StatusBadge<G>
where
    G: StatusGetter,
{
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let (text, color) = self.getter.get_status(state);
        let style = Style::default().fg(color);
        let spans = Spans(vec![Span::styled(text, style)]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
