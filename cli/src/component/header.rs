use crate::component::elements::block_with_title;
use crate::component::logo::Logo;
use crate::component::mode::ModeSelector;
use crate::component::{Component, Focus, Frame, Input};
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders};

pub struct Header {
    logo: Logo,
    mode_selector: ModeSelector,
}

impl Header {
    pub fn new() -> Self {
        Self {
            logo: Logo::new(),
            mode_selector: ModeSelector::new(),
        }
    }
}

impl Input for Header {
    fn on_input(&mut self, key: KeyCode) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for Header {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let constraints = [Constraint::Percentage(40), Constraint::Percentage(60)];
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        self.logo.draw(f, chunks[0]);
        self.mode_selector.draw(f, chunks[1]);
    }
}
