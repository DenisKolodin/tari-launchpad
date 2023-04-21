pub mod main_view;
pub mod tabs;

use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Widget;
use tui::Frame;

pub enum Outcome {
    Out,
    Next,
    Prev,
}

pub trait Component<B: Backend> {
    fn update(&mut self, key: KeyCode) -> Option<Outcome>;
    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect);
}
