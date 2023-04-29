use crate::component::elements::block_with_title;
use crate::component::tabs::AppTabs;
use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum ExpertTabs {
    Performance,
    Containers,
    Logs,
}

pub struct ExpertScene {
    expert_tabs: AppTabs<ExpertTabs>,
}

impl ExpertScene {
    pub fn new() -> Self {
        Self {
            expert_tabs: AppTabs::new(),
        }
    }
}

impl Input for ExpertScene {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        self.expert_tabs.on_input(key);
        None
    }
}

impl<B: Backend> Component<B> for ExpertScene {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.expert_tabs.draw(f, chunks[0], state);
    }
}
