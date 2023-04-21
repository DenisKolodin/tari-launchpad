use crate::component::{Component, ComponentContext};
use crossterm::event::KeyCode;
use std::io::Stdout;
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};
use tui::{
    backend::CrosstermBackend,
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

impl<T> AppTabs<T> {}

impl<T> Component for AppTabs<T>
where
    T: IntoEnumIterator + Copy + Into<usize> + ToString,
{
    fn update(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {}
            KeyCode::Down | KeyCode::Char('j') => {}
            KeyCode::Left | KeyCode::Char('h') => {}
            KeyCode::Right | KeyCode::Char('l') => {}
            _ => {}
        }
    }

    fn render<'a>(&self, rect: Rect, ctx: &mut ComponentContext<'a>) {
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
        ctx.render(tabs);
    }
}
