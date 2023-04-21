use crate::component::{Component, Outcome};
use crossterm::event::KeyCode;
use std::io::Stdout;
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
}

impl<T> AppTabs<T> {}

impl<B, T> Component<B> for AppTabs<T>
where
    B: Backend,
    T: IntoEnumIterator + Copy + Into<usize> + ToString,
{
    fn update(&mut self, key: KeyCode) -> Option<Outcome> {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {}
            KeyCode::Down | KeyCode::Char('j') => {}
            KeyCode::Left | KeyCode::Char('h') => {}
            KeyCode::Right | KeyCode::Char('l') => {}
            _ => {}
        }
        None
    }

    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let titles = T::iter()
            .map(|s| Spans::from(vec![Span::raw(s.to_string())]))
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.selected_tab.into())
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        f.render_widget(tabs, rect);
    }
}
