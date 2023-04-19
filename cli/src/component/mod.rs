pub mod tabs;

use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::Frame;

pub trait Component {
    fn update(&mut self, key: KeyCode);
    fn render<'f>(&self, rect: Rect, f: &mut Frame<'f, CrosstermBackend<Stdout>>);
}
