use crate::component::{elements::block_with_title, Component, ComponentEvent, Input, Pass};
use crate::state::{AppState, Focus};
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
    focus_on: Focus,
    focus_to: Focus,
    selected: usize,
    items: Vec<T>,
}

impl<T> AppTabs<T>
where
    T: IntoEnumIterator,
{
    pub fn new(focus_to: Focus) -> Self {
        Self {
            focus_on: Focus::Root,
            focus_to,
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

    fn next(&mut self) {
        let index = self.selected + 1;
        if self.items.get(index).is_some() {
            self.selected = index;
        } else {
            self.selected = 0;
        }
    }

    fn prev(&mut self) {
        if self.selected > 0 {
            let index = self.selected - 1;
            self.selected = index;
        } else {
            self.selected = self.items.len() - 1;
        }
    }
}

impl<T> Input for AppTabs<T> {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == self.focus_on {
            match event.pass() {
                Pass::Next | Pass::Right => {
                    self.next();
                }
                Pass::Left => {
                    self.prev();
                }
                Pass::Down | Pass::Enter => {
                    state.focus_on(self.focus_to);
                }
                _ => {}
            }
        }
    }
}

impl<B, T> Component<B> for AppTabs<T>
where
    B: Backend,
    T: IntoEnumIterator + Copy + ToString,
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let _tag_style = Style::default().fg(Color::Rgb(4, 209, 144));
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
        let block = block_with_title(None, state.focus_on == self.focus_on);
        let tabs = Tabs::new(titles)
            .block(block)
            .select(self.selected)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Magenta));
        f.render_widget(tabs, rect);
    }
}
