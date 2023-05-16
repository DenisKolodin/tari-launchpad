mod message;

use crate::component::elements::{block_with_title, logo};
use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, BaseNodeFocus, Focus, MiningFocus, WalletFocus};

use message::MessageWidget;
use std::time::{Duration, Instant};
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

const MSG_1: &str = "
Hi! My name is T-Bot. It is a great pleasure and an honor to meet you!
I have no memory of human faces, so if our paths have already crossed in the Aurora app, I’m glad to see you again!
";

pub struct OnboardingScene {
    messages: Vec<MessageWidget>,
    wink: Option<Instant>,
}

impl OnboardingScene {
    pub fn new() -> Self {
        let message = MessageWidget::new(MSG_1);
        Self {
            messages: vec![message],
            wink: Some(Instant::now()),
        }
    }
}

impl Input for OnboardingScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if let Some(wink) = self.wink {
            if wink.elapsed() >= Duration::from_secs(5) {
                self.wink.take();
                state.redraw();
            }
        } else {
            self.wink = Some(Instant::now());
            state.redraw();
        }
    }
}

impl<B: Backend> Component<B> for OnboardingScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(rect);
        let constraints = [
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(h_chunks[1]);

        let constraints = [Constraint::Min(0), Constraint::Length(3)];
        let view_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(v_chunks[1]);

        if let Some(message) = self.messages.last() {
            message.draw(f, view_chunks[0], state);
        }

        let constraints = [Constraint::Min(0), Constraint::Length(5)];
        let line_chinks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(view_chunks[1]);

        let style = Style::default().fg(Color::White); //.bg(Color::Magenta);
        let bot_state = if self.wink.is_some() {
            "[o o]"
        } else {
            "[- -]"
        };
        let text = vec![Spans::from(Span::styled(bot_state, style))];
        let bot = Paragraph::new(text);
        f.render_widget(bot, line_chinks[1]);
    }
}
