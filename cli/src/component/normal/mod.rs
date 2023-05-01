mod mining;

use crate::component::tabs::AppTabs;
use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::LaunchpadState;
use crossterm::event::Event;
use mining::MiningScene;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum NormalTabs {
    Mining,
    #[strum(serialize = "Base Node")]
    BaseNode,
    Wallet,
}

pub struct NormalScene {
    normal_tabs: AppTabs<NormalTabs>,
    mining_scene: MiningScene,
}

impl NormalScene {
    pub fn new() -> Self {
        Self {
            normal_tabs: AppTabs::new(),
            mining_scene: MiningScene::new(),
        }
    }
}

impl Input for NormalScene {
    fn on_event(&mut self, event: ComponentEvent) -> Option<MoveFocus> {
        self.normal_tabs.on_event(event);
        None
    }
}

impl<B: Backend> Component<B> for NormalScene {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.normal_tabs.draw(f, chunks[0], state);
        match self.normal_tabs.selected() {
            NormalTabs::Mining => {
                self.mining_scene.draw(f, chunks[1], state);
            }
            NormalTabs::BaseNode => {}
            NormalTabs::Wallet => {}
        }
    }
}
