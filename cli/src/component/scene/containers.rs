use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};

pub struct Containers {}

impl Containers {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Containers {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for Containers {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let block = block_with_title("Containers");
        f.render_widget(block, rect);
    }
}
