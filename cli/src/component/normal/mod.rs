mod base_node;
mod chrono_button;
mod hint;
mod mining;
mod wallet;

use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, BaseNodeFocus, Focus, MiningFocus, WalletFocus};

use base_node::BaseNodeScene;
use mining::MiningScene;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use wallet::WalletScene;

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

    fn focus_to(&self, _: &AppState) -> Focus {
        match self {
            Self::Mining => Focus::Mining(MiningFocus::TariMining),
            Self::BaseNode => Focus::BaseNode(BaseNodeFocus::BaseNode),
            // TODO: Use the high-level focus
            Self::Wallet => Focus::Wallet(WalletFocus::Password),
        }
    }
}

pub struct NormalScene {
    normal_tabs: AppTabs<NormalTabs>,
    mining_scene: MiningScene,
    base_node_scene: BaseNodeScene,
    wallet_scene: WalletScene,
}

impl NormalScene {
    pub fn new() -> Self {
        Self {
            normal_tabs: AppTabs::new(),
            mining_scene: MiningScene::new(),
            base_node_scene: BaseNodeScene::new(),
            wallet_scene: WalletScene::new(),
        }
    }
}

impl Input for NormalScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        let _focus = self.normal_tabs.on_event(event, state);
        match self.normal_tabs.selected() {
            NormalTabs::Mining => {
                self.mining_scene.on_event(event, state);
            }
            NormalTabs::BaseNode => {
                self.base_node_scene.on_event(event, state);
            }
            NormalTabs::Wallet => {
                self.wallet_scene.on_event(event, state);
            }
        }
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
            NormalTabs::BaseNode => {
                self.base_node_scene.draw(f, chunks[1], state);
            }
            NormalTabs::Wallet => {
                self.wallet_scene.draw(f, chunks[1], state);
            }
        }
    }
}
