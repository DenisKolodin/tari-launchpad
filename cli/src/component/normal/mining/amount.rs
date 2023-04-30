use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub trait AmountGetter {
    fn get_amount(&self, state: &LaunchpadState) -> (u64, &str);
}

pub struct AmountIndicator<G> {
    getter: G,
}

impl<G> AmountIndicator<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for AmountIndicator<G> {
    fn on_input(&mut self, _key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend, G> Component<B> for AmountIndicator<G>
where
    G: AmountGetter,
{
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {}
}
