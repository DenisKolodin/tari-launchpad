use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::AppState;

use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub struct HintLine {}

impl HintLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for HintLine {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for HintLine {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let mining = false; // TODO: Get it from the state
        let text = if mining {
            "Awesome! Tari Mining is on."
        } else {
            "You are one step away from staring mining."
        };
        let spans = Spans(vec![Span::raw(text)]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
