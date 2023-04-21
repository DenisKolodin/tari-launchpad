use crate::component::{Component, Frame, Input, Move};
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};

pub struct ContainersScene {}

impl ContainersScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for ContainersScene {
    fn on_input(&mut self, key: KeyCode) -> Option<Move> {
        None
    }
}

impl<B: Backend> Component<B> for ContainersScene {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, rect);
    }
}
