use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use rust_decimal::Decimal;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub trait AmountGetter {
    fn get_amount(&self, state: &LaunchpadState) -> (Decimal, &str);
}

pub struct AmountIndicator<G> {
    getter: G,
}

impl<G> AmountIndicator<G> {
    pub fn new(getter: G) -> Self {
        Self { getter }
    }
}

impl<G> Input for AmountIndicator<G> {
    fn on_event(&mut self, _event: ComponentEvent) -> Option<MoveFocus> {
        None
    }
}

impl<B: Backend, G> Component<B> for AmountIndicator<G>
where
    G: AmountGetter,
{
    type State = LaunchpadState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let (amount, curr) = self.getter.get_amount(state);
        let s = amount.to_string();

        let spans = Spans(vec![
            Span::raw(s),
            Span::raw(" "),
            Span::styled(curr, Style::default().add_modifier(Modifier::BOLD)),
        ]);
        let text = vec![spans];
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
