use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph};

pub enum Mode {
    Normal,
    Expert,
    Settings,
}

/// A selector to switch between `Normal`, `Expert`, and `Settings`.
pub struct ModeSelector {
    expert: bool,
    settings: bool,
}

impl ModeSelector {
    pub fn new() -> Self {
        Self {
            expert: false,
            settings: false,
        }
    }

    pub fn selected(&self) -> Mode {
        match (self.expert, self.settings) {
            (_, true) => Mode::Settings,
            (true, false) => Mode::Expert,
            (false, false) => Mode::Normal,
        }
    }
}

impl Input for ModeSelector {
    fn on_input(&mut self, key: KeyEvent) -> Option<Focus> {
        None
    }
}

impl<B: Backend> Component<B> for ModeSelector {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let not_selected = Style::default().fg(Color::White);
        let selected = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let spans = Spans(vec![
            Span::styled("Normal", not_selected),
            Span::raw(" ( o) "),
            Span::styled("Expert", selected),
            Span::raw(" | "),
            Span::styled("Settings", not_selected),
        ]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Right);
        f.render_widget(paragraph, rect);
    }
}
