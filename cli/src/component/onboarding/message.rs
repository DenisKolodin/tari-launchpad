use crate::component::elements::{block_with_title, logo};
use crate::component::tabs::{AppTabs, TabGetter};
use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, BaseNodeFocus, Focus, MiningFocus, WalletFocus};

use strum::{Display, EnumCount, EnumIter, FromRepr};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::widgets::{Paragraph, Wrap};

pub struct MessageWidget {
    text: String,
}

impl MessageWidget {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.trim().into(),
        }
    }
}

impl Input for MessageWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {}
}

impl<B: Backend> Component<B> for MessageWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(None, false);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .horizontal_margin(3)
            .vertical_margin(1)
            .split(inner_rect);
        let paragraph = Paragraph::new(self.text.as_ref()).wrap(Wrap { trim: false });
        f.render_widget(paragraph, chunks[0]);
    }
}
