use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub struct MiningTip {}

impl MiningTip {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for MiningTip {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for MiningTip {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
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
