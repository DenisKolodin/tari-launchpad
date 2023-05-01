use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::AppState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub struct MiningTip {}

impl MiningTip {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for MiningTip {
    fn on_event(&mut self, _event: ComponentEvent) -> Option<MoveFocus> {
        None
    }
}

impl<B: Backend> Component<B> for MiningTip {
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