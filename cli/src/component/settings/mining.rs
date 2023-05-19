use crate::component::elements::block_with_title;
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::AppState;

use tui::backend::Backend;
use tui::layout::Rect;

pub struct MiningSettings {}

impl MiningSettings {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for MiningSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for MiningSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(Some("Mining Settings"), false);
        f.render_widget(block, rect);
    }
}
