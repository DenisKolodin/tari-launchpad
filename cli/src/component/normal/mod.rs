mod base_node;
mod mining;

use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, Focus};

use mining::MiningScene;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum NormalTabs {
    Mining,
    #[strum(serialize = "Base Node")]
    BaseNode,
    Wallet,
}

impl TabGetter for NormalTabs {
    fn get_badge(&self, state: &AppState) -> Option<(&str, Color)> {
        match self {
            Self::Mining => {
                if state.merged_mining.is_active() || state.tari_mining.is_active() {
                    return Some(("(running)", Color::Green));
                }
            }
            _ => {}
        }
        None
    }
}

pub struct NormalScene {
    normal_tabs: AppTabs<NormalTabs>,
    mining_scene: MiningScene,
}

impl NormalScene {
    pub fn new() -> Self {
        Self {
            normal_tabs: AppTabs::new(Focus::TariMining),
            mining_scene: MiningScene::new(),
        }
    }
}

impl Input for NormalScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        let _focus = self.normal_tabs.on_event(event, state);
        self.mining_scene.on_event(event, state);
    }
}

impl<B: Backend> Component<B> for NormalScene {
    type State = AppState;

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
