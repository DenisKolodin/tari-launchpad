use crate::component::elements::block_with_title;
use crate::component::labeled_input::LabeledInput;
use crate::component::widgets::separator::Separator;
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::AppState;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct DockerSettings {
    docker_tag: LabeledInput,
    docker_registry: LabeledInput,
}

impl DockerSettings {
    pub fn new() -> Self {
        Self {
            docker_tag: LabeledInput::new("Docker Tag"),
            docker_registry: LabeledInput::new("Docker Registry"),
        }
    }
}

impl Input for DockerSettings {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend> Component<B> for DockerSettings {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Docker Settings"), false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);
        let constraints = [
            // Expert
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            // Image Statuses
            Constraint::Length(1),
            Constraint::Min(0),
        ];
        let chunks = Layout::default()
            .vertical_margin(1)
            .horizontal_margin(3)
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        let sep = Separator::new("Expert");
        f.render_widget(sep, chunks[0]);
        self.docker_tag.draw(f, chunks[1], state);
        self.docker_registry.draw(f, chunks[2], state);

        let sep = Separator::new("Expert");
        f.render_widget(sep, chunks[3]);
    }
}
