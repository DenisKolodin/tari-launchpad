mod message;

use crate::component::elements::{block_with_title, logo};
use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input, Pass};
use crate::state::{AppState, BaseNodeFocus, Focus, MiningFocus, WalletFocus};

use message::MessageWidget;
use std::time::{Duration, Instant};
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Gauge, Paragraph};

const MSG_1: &str = "
Hi! My name is T-Bot. It is a great pleasure and an honor to meet you!
I have no memory of human faces, so if our paths have already crossed in the Aurora app, I’m glad to see you again!
";

const MSG_2: &str = "
I'm kind of like Gandalf, Dumbledore or Obi-Wan Kenobi. You know, the guy who makes sure the novice gets to a certain destination. Spoiler alert: in this saga the guide will survive. Regardless of whether this is your first contact with cryptocurrencies or you are advanced in it, I will stay with you until the Tari Launchpad setup process is successfully completed.
";

const MSG_3: &str = "
So let's get started! 🚀 The setup process usually takes 5 to 10 minutes. A duo like you and me should be able to deal with it quickly, right?
";

pub struct OnboardingScene {
    messages: Vec<MessageWidget>,
    wink: Option<Instant>,
}

impl OnboardingScene {
    pub fn new() -> Self {
        let msg_1 = MessageWidget::new(MSG_1);
        let msg_2 = MessageWidget::new(MSG_2);
        let msg_3 = MessageWidget::new(MSG_3);
        Self {
            messages: vec![msg_1, msg_2, msg_3],
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

        match event.pass() {
            Pass::Leave => {
                state.focus_on(Focus::Root);
            }
            _ => {}
        }
    }
}

impl OnboardingScene {
    fn get_progress(&self, state: &AppState) -> u16 {
        100
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

        let constraints = [Constraint::Min(0), Constraint::Length(1)];
        let view_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(v_chunks[1]);

        if let Some(message) = self.messages.last() {
            message.draw(f, view_chunks[0], state);
        }

        let constraints = [
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Length(5),
        ];
        let line_chinks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(view_chunks[1]);

        let gauge = Gauge::default()
            .label("")
            .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Reset))
            .percent(self.get_progress(state));
        f.render_widget(gauge, line_chinks[0]);
        let style = Style::default().fg(Color::White);
        let bot_state = if self.wink.is_some() {
            "[o o]"
        } else {
            "[- -]"
        };
        let text = vec![Spans::from(Span::styled(bot_state, style))];
        let bot = Paragraph::new(text);
        f.render_widget(bot, line_chinks[2]);
    }
}
