use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph};

/// A button with a clock.
pub struct ChronoButton {}

impl ChronoButton {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for ChronoButton {
    fn on_event(&mut self, _event: ComponentEvent) -> Option<MoveFocus> {
        None
    }
}

impl<B: Backend> Component<B> for ChronoButton {
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        let block = Block::default(); //.style(Style::default().bg(Color::Magenta));
        let inner_rect = block.inner(v_chunks[0]);
        f.render_widget(block, v_chunks[0]);

        let spans = Spans(vec![Span::styled(
            "  Set up & start mining  ",
            Style::default().bg(Color::Magenta),
        )]);
        let text = vec![spans];
        let p = Paragraph::new(text);
        f.render_widget(p, inner_rect);
    }
}
