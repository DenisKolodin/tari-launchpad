pub mod containers_scene;
pub mod elements;
pub mod main_view;
pub mod tabs;

use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub trait Component<B: Backend> {
    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect);
}

pub enum Move {
    Out,
    Up,
    Down,
    Next,
    Prev,
}

pub trait Input {
    fn on_input(&mut self, key: KeyCode) -> Option<Move>;
}
