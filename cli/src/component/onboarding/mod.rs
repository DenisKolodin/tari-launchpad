mod message;

use crate::component::elements::{block_with_title, logo};
use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, BaseNodeFocus, Focus, MiningFocus, WalletFocus};

use message::MessageWidget;
use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;

const BOT: &str = "
╓   ╖
║O O║
╙   ╜
";
const MSG_1: &str = "
Hi! My name is T-Bot. It is a great pleasure and an honor to meet you!
I have no memory of human faces, so if our paths have already crossed in the Aurora app, I’m glad to see you again!
";

pub struct OnboardingScene {
    messages: Vec<MessageWidget>,
}

impl OnboardingScene {
    pub fn new() -> Self {
        let message = MessageWidget::new(MSG_1);
        Self {
            messages: vec![message],
        }
    }
}

impl Input for OnboardingScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {}
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
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(h_chunks[1]);
        if let Some(message) = self.messages.last() {
            message.draw(f, v_chunks[1], state);
        }
    }
}
