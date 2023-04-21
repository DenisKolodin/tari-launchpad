use crate::component::tabs::{AppTab, AppTabs};
use crate::component::{Component, ComponentContext};
use crossterm::event::KeyCode;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct MainView {
    tabs: AppTabs<AppTab>,
}

impl Component for MainView {
    fn update(&mut self, key: KeyCode) {
        self.tabs.update(key);
    }

    fn render<'a>(&self, rect: Rect, ctx: &mut ComponentContext<'a>) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(rect);
        self.tabs.render(main_chunks[0], ctx);
    }
}
