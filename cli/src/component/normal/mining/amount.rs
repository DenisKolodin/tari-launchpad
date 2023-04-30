use crate::component::{Component, Focus, Frame, Input};
use crate::state::LaunchpadState;
use crossterm::event::KeyEvent;
use num_format::{CustomFormat, Locale, ToFormattedString};
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub trait AmountGetter {
    fn get_amount(&self, state: &LaunchpadState) -> (u64, &str);
}

pub struct AmountIndicator<G> {
    getter: G,
    format: CustomFormat,
}

impl<G> AmountIndicator<G> {
    pub fn new(getter: G) -> Self {
        let format = CustomFormat::builder()
            .separator(" ")
            .build()
            .unwrap_or_default();
        Self { getter, format }
    }
}

impl<G> Input for AmountIndicator<G> {
    fn on_input(&mut self, _key: KeyEvent) -> Option<Focus> {
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
        let s = amount.to_formatted_string(&self.format);

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
