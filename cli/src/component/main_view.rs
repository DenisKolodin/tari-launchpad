use crate::component::containers_scene::ContainersScene;
use crate::component::tabs::{AppTab, AppTabs};
use crate::component::{Component, Input, Move};
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct MainView {
    tabs: AppTabs<AppTab>,
    containers_scene: ContainersScene,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            tabs: AppTabs::new(AppTab::Containers),
            containers_scene: ContainersScene::new(),
        }
    }
}

impl Input for MainView {
    fn on_input(&mut self, key: KeyCode) -> Option<Move> {
        self.tabs.on_input(key);
        match self.tabs.selected() {
            AppTab::Containers => {}
            AppTab::Wallet => {}
        }
        None
    }
}

impl<B: Backend> Component<B> for MainView {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(rect);
        self.tabs.draw(f, main_chunks[0]);
        match self.tabs.selected() {
            AppTab::Containers => {
                self.containers_scene.draw(f, main_chunks[1]);
            }
            AppTab::Wallet => {}
        }
    }
}
