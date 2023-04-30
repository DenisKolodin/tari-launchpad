mod tip;

use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;

use tip::MiningTip;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct MiningScene {
    mining_tip: MiningTip,
}

impl MiningScene {
    pub fn new() -> Self {
        Self {
            mining_tip: MiningTip::new(),
        }
    }
}

impl Input for MiningScene {
    fn on_input(&mut self, _key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for MiningScene {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.mining_tip.draw(f, chunks[0], state);
        // let block = block_with_title(None);
        // f.render_widget(block, rect);
    }
}
