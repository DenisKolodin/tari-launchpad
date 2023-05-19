mod mining;

use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, Focus};
use mining::MiningSettings;

use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum SettingsTabs {
    Mining,
    Wallet,
    BaseNode,
    Docker,
    Logs,
    Security,
}

impl TabGetter for SettingsTabs {
    fn focus_to(&self, _: &AppState) -> Focus {
        Focus::Root
    }
}

pub struct SettingsScene {
    settings_tabs: AppTabs<SettingsTabs>,
    mining_settings: MiningSettings,
}

impl SettingsScene {
    pub fn new() -> Self {
        Self {
            settings_tabs: AppTabs::new(),
            mining_settings: MiningSettings::new(),
        }
    }
}

impl Input for SettingsScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.settings_tabs.on_event(event, state);
    }
}

impl<B: Backend> Component<B> for SettingsScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.settings_tabs.draw(f, chunks[0], state);
    }
}
