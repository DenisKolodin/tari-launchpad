use crate::component::elements::block_with_title;
use crate::component::{Component, Focus, Frame, Input};
use crossterm::event::KeyModifiers;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph};

#[derive(Debug, PartialEq, Eq)]
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
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('n') => {
                    self.expert = false;
                    self.settings = false;
                }
                KeyCode::Char('e') => {
                    self.expert = !self.expert;
                    self.settings = false;
                }
                KeyCode::Char('s') => {
                    self.settings = !self.settings;
                }
                _ => {}
            }
        }
        None
    }
}

impl<B: Backend> Component<B> for ModeSelector {
    fn draw(&self, f: &mut Frame<B>, rect: Rect) {
        let not_selected = Style::default().fg(Color::White);
        let selected = Style::default().fg(Color::Magenta);
        let bold = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let style_for = |mode: Mode| -> Style {
            if mode == self.selected() {
                selected
            } else {
                not_selected
            }
        };
        let selector = if self.expert { " o" } else { "o " };
        let spans = Spans(vec![
            Span::styled("Normal", style_for(Mode::Normal)),
            Span::raw(" ("),
            Span::styled(selector, bold),
            Span::raw(") "),
            Span::styled("Expert", style_for(Mode::Expert)),
            Span::raw(" | "),
            Span::styled("Settings", style_for(Mode::Settings)),
        ]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Right);
        f.render_widget(paragraph, rect);
    }
}
