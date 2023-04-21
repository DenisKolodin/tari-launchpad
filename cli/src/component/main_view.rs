use crate::component::tabs::{AppTab, AppTabs};
use crate::component::{Component, Input, Move};
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct MainView {
    tabs: AppTabs<AppTab>,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            tabs: AppTabs::new(AppTab::Containers),
        }
    }
}

impl Input for MainView {
    fn on_input(&mut self, key: KeyCode) -> Option<Move> {
        self.tabs.on_input(key);
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
    }
}
