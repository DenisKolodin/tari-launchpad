mod elements;
mod header;
mod logo;
mod main_view;
mod mode;
mod scene;
mod tabs;

pub use main_view::MainView;

use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub trait Component<B: Backend> {
    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect);
}

pub enum Focus {
    /// Entering into a component.
    In,
    /// Exiting out of a component.
    Out,
    Up,
    Down,
    Next,
    Prev,
}

pub trait Input {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus>;
}
