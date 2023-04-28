use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;

pub struct Wallet {}

impl Wallet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Wallet {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for Wallet {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Wallet"));
        f.render_widget(block, rect);
    }
}
