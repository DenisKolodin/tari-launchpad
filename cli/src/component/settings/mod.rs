use crate::component::tabs::AppTabs;
use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
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

pub struct SettingsScene {
    settings_tabs: AppTabs<SettingsTabs>,
}

impl SettingsScene {
    pub fn new() -> Self {
        Self {
            settings_tabs: AppTabs::new(),
        }
    }
}

impl Input for SettingsScene {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        self.settings_tabs.on_input(key);
        None
    }
}

impl<B: Backend> Component<B> for SettingsScene {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(3), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.settings_tabs.draw(f, chunks[0], state);
    }
}
