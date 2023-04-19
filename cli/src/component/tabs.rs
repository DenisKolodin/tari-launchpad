use crate::component::Component;
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

pub struct AppTabs {
    tab: AppTab,
}

impl Component for AppTabs {
    fn update(&mut self, key: KeyCode) {}
    fn render<'f>(&self, rect: Rect, f: &mut Frame<'f, CrosstermBackend<Stdout>>) {
        let titles = AppTab::iter()
            .map(|s| Spans::from(vec![Span::raw(s.to_string())]))
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            //.select(self.dashboard_state.selected_tab as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        f.render_widget(tabs, rect);
    }
}
