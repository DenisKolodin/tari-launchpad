pub mod main_view;
pub mod tabs;

use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::Widget;
use tui::Frame;

pub trait Component {
    fn update(&mut self, key: KeyCode);
    fn render<'a>(&self, ctx: &mut ComponentContext<'a>);
}

pub struct ComponentContext<'a> {
    pub rect: Rect,
    pub f: &'a mut Frame<'a, CrosstermBackend<Stdout>>,
    pub active: bool,
}

impl<'a> ComponentContext<'a> {
    pub fn render<W: Widget>(&mut self, widget: W) {
        self.f.render_widget(widget, self.rect);
    }
}
