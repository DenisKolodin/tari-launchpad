use crate::component::{elements::block_with_title, Component, Input, Move};
use crossterm::event::KeyCode;
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum AppTab {
    Containers,
    Wallet,
}

impl From<AppTab> for usize {
    fn from(tab: AppTab) -> Self {
        tab as Self
    }
}

pub struct AppTabs<T> {
    selected_tab: T,
}

impl<T> AppTabs<T> {
    pub fn new(selected_tab: T) -> Self {
        Self { selected_tab }
    }

    pub fn selected(&self) -> &T {
        &self.selected_tab
    }
}

impl<T> Input for AppTabs<T> {
    fn on_input(&mut self, key: KeyCode) -> Option<Move> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {}
            KeyCode::Down | KeyCode::Char('j') => {}
            KeyCode::Left | KeyCode::Char('h') => {}
            KeyCode::Right | KeyCode::Char('l') => {}
            _ => {}
        }
        None
    }
}

impl<B, T> Component<B> for AppTabs<T>
where
    B: Backend,
    T: IntoEnumIterator + Copy + Into<usize> + ToString,
{
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let titles = T::iter()
            .map(|s| Spans::from(vec![Span::raw(s.to_string())]))
            .collect();
        let block = block_with_title("Tabs");
        let tabs = Tabs::new(titles)
            .block(block)
            .select(self.selected_tab.into())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Magenta));
        f.render_widget(tabs, rect);
    }
}
