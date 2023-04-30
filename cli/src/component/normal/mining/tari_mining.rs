use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct TariMiningWidget {}

impl TariMiningWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for TariMiningWidget {
    fn on_input(&mut self, _key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for TariMiningWidget {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Tari Mining"));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
    }
}
