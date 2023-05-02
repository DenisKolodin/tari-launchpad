use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::AppState;
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph};

pub trait ChronoGetter {
    /// How long the mining is active.
    fn get_duration(&self, state: &AppState) -> Option<Duration>;
}

/// A button with a clock.
pub struct ChronoButton<G> {
    getter: G,
}

impl<G> ChronoButton<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for ChronoButton<G> {
    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) {}
}

impl<B: Backend, G> Component<B> for ChronoButton<G>
where
    G: ChronoGetter,
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Min(0)];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        let block = Block::default(); //.style(Style::default().bg(Color::Magenta));
        let inner_rect = block.inner(v_chunks[0]);
        f.render_widget(block, v_chunks[0]);

        let caption;
        if let Some(dur) = self.getter.get_duration(state) {
            let total = dur.as_secs();
            let seconds = total % 60;
            let total = total / 60;
            let minutes = total % 60;
            let hours = total / 60;
            caption = format!("  {:02}:{:02}:{:02} | Pause  ", hours, minutes, seconds);
        } else {
            caption = "  Start mining  ".to_string();
        }

        let spans = Spans(vec![Span::styled(
            // "  Set up & start mining  ",
            // "  Start mining  ",
            caption,
            Style::default().bg(Color::Magenta),
        )]);
        let text = vec![spans];
        let p = Paragraph::new(text);
        f.render_widget(p, inner_rect);
    }
}
