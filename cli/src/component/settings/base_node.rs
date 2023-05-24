use crate::component::elements::block_with_title;
use crate::component::labeled_input::LabeledInput;
use crate::component::widgets::separator::Separator;
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::AppState;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct BaseNodeSettings {
    root_folder: LabeledInput,
}

impl BaseNodeSettings {
    pub fn new() -> Self {
        Self {
            root_folder: LabeledInput::new("Root folder"),
        }
    }
}

impl Input for BaseNodeSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for BaseNodeSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("BaseNode Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [Constraint::Length(1), Constraint::Min(3)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        let sep = Separator::new("Expert");
        f.render_widget(sep, chunks[0]);
        self.root_folder.draw(f, chunks[1], state);
    }
}
