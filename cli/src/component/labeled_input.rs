use crate::component::widgets::label::Label;
use crate::component::{Component, ComponentEvent, Frame, Input, Pass};
use crate::state::{AppState, Focus, MiningFocus};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui_textarea::TextArea;

pub struct LabeledInput {
    label: String,
    input: TextArea<'static>,
}

impl LabeledInput {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            input: TextArea::default(),
        }
    }
}

impl<B: Backend> Component<B> for LabeledInput {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Percentage(40), Constraint::Percentage(60)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let label = Label::new(&self.label);
        f.render_widget(label, h_chunks[0]);
        let input = self.input.widget();
        f.render_widget(input, h_chunks[1]);
    }
}
