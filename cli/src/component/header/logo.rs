use crate::component::{Component, Frame};
use crate::state::AppState;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub struct Logo {}

impl Logo {
    pub fn new() -> Self {
        Self {}
    }
}

impl<B: Backend> Component<B> for Logo {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let bold = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let spans = Spans(vec![
            Span::styled("Tari", bold),
            Span::raw(" "),
            Span::styled("Launchpad", bold),
            Span::raw(" "),
            Span::styled("App", bold),
        ]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
