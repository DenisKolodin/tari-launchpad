use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};

pub struct Wallet {}

impl Wallet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Wallet {
    fn on_input(&mut self, key: KeyCode) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for Wallet {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let block = block_with_title("Wallet");
        f.render_widget(block, rect);
    }
}
