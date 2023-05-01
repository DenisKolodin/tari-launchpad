use crate::component::elements::block_with_title;
use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;

pub struct Containers {}

impl Containers {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for Containers {
    fn on_event(&mut self, _event: ComponentEvent) -> Option<MoveFocus> {
        None
    }
}

impl<B: Backend> Component<B> for Containers {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(Some("Containers"));
        f.render_widget(block, rect);
    }
}
