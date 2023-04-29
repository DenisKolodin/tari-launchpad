use crate::component::{elements::block_with_title, Component, Focus, Input};
use crate::state::LaunchpadState;
use crossterm::event::{KeyCode, KeyEvent};
use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Tabs,
    Frame,
};

pub struct AppTabs<T> {
    selected: usize,
    items: Vec<T>,
}

impl<T> AppTabs<T>
where
    T: IntoEnumIterator,
{
    pub fn new() -> Self {
        Self {
            selected: 0,
            items: T::iter().collect(),
        }
    }
}

impl<T> AppTabs<T> {
    pub fn selected(&self) -> &T {
        self.items
            .get(self.selected)
            .expect("the selected tab is out of the range (empty tabs list)")
    }

    fn next(&mut self) -> bool {
        let index = self.selected + 1;
        if self.items.get(index).is_some() {
            self.selected = index;
            true
        } else {
            false
        }
    }

    fn prev(&mut self) -> bool {
        if self.selected > 0 {
            let index = self.selected - 1;
            self.selected = index;
            true
        } else {
            false
        }
    }
}

impl<T> Input for AppTabs<T> {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        let mut move_to = None;
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                move_to = Some(Focus::Up);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                move_to = Some(Focus::Down);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if !self.prev() {
                    move_to = Some(Focus::Prev);
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.next() {
                    move_to = Some(Focus::Next);
                }
            }
            _ => {}
        }
        move_to
    }
}

impl<B, T> Component<B> for AppTabs<T>
where
    B: Backend,
    T: IntoEnumIterator + Copy + ToString,
{
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let tag_style = Style::default().fg(Color::Rgb(4, 209, 144));
        let titles = self
            .items
            .iter()
            .map(|s| {
                Spans::from(vec![
                    Span::raw(s.to_string()),
                    // Span::styled(" (running)", tag_style),
                ])
            })
            .collect();
        let block = block_with_title(None);
        let tabs = Tabs::new(titles)
            .block(block)
            .select(self.selected)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Magenta));
        f.render_widget(tabs, rect);
    }
}
