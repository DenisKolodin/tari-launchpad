use crate::component::expert::ExpertTabs;
use crate::component::header::Header;
use crate::component::mode::Mode;
use crate::component::normal::NormalTabs;
use crate::component::scene;
use crate::component::settings::SettingsTabs;
use crate::component::tabs::{AppTab, AppTabs};
use crate::component::{Component, Focus, Input};
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct MainView {
    header: Header,
    normal_tabs: AppTabs<NormalTabs>,
    expert_tabs: AppTabs<ExpertTabs>,
    settings_tabs: AppTabs<SettingsTabs>,
    containers_scene: scene::Containers,
    wallet_scene: scene::Wallet,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            normal_tabs: AppTabs::new(),
            expert_tabs: AppTabs::new(),
            settings_tabs: AppTabs::new(),
            containers_scene: scene::Containers::new(),
            wallet_scene: scene::Wallet::new(),
        }
    }
}

impl Input for MainView {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        self.header.on_input(key);
        /*
        self.tabs.on_input(key);
        match self.tabs.selected()? {
            AppTab::Containers => {}
            AppTab::Wallet => {}
        }
        */
        None
    }
}

impl<B: Backend> Component<B> for MainView {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.header.draw(f, chunks[0]);
        match self.header.mode_selector.selected() {
            Mode::Normal => {
                self.normal_tabs.draw(f, chunks[1]);
            }
            Mode::Expert => {
                self.expert_tabs.draw(f, chunks[1]);
            }
            Mode::Settings => {
                self.settings_tabs.draw(f, chunks[1]);
            }
        }
        // self.tabs.draw(f, chunks[1]);
        /*
        match self.tabs.selected() {
            Some(AppTab::Containers) => {
                self.containers_scene.draw(f, chunks[2]);
            }
            Some(AppTab::Wallet) => {
                self.wallet_scene.draw(f, chunks[2]);
            }
            None => {}
        }
        */
    }
}
