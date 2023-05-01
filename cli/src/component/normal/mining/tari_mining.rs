use crate::component::elements::{block_with_title, logo};
use crate::component::normal::mining::amount::{AmountGetter, AmountIndicator};
use crate::component::normal::mining::chrono_button::ChronoButton;
use crate::component::normal::mining::status_badge::{StatusBadge, StatusGetter};
use crate::component::{Component, ComponentEvent, Frame, Input, MoveFocus};
use crate::state::AppState;
use crossterm::event::KeyEvent;
use rust_decimal::Decimal;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;

const LOGO: &str = r#"
╔╦╗┌─┐┬─┐┬  ╔╦╗┬┌┐┌┬┌┐┌┌─┐
 ║ ├─┤├┬┘│  ║║║│││││││││ ┬
 ╩ ┴ ┴┴└─┴  ╩ ╩┴┘└┘┴┘└┘└─┘
"#;

struct TariMiningGetter;

impl StatusGetter for TariMiningGetter {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.tari_mining.is_active {
            ("(Running)", Color::Green)
        } else {
            ("(Ready to set)", Color::Cyan)
        }
    }
}

struct XtrGetter;

impl AmountGetter for XtrGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str) {
        let amount = state.tari_mining.tari_amount;
        (amount, "XTR")
    }
}

pub struct TariMiningWidget {
    status_badge: StatusBadge<TariMiningGetter>,
    tari_amount: AmountIndicator<XtrGetter>,
    button: ChronoButton,
}

impl TariMiningWidget {
    pub fn new() -> Self {
        Self {
            status_badge: StatusBadge::new(TariMiningGetter),
            tari_amount: AmountIndicator::new(XtrGetter),
            button: ChronoButton::new(),
        }
    }
}

impl Input for TariMiningWidget {
    fn on_event(&mut self, _event: ComponentEvent) -> Option<MoveFocus> {
        None
    }
}

impl<B: Backend> Component<B> for TariMiningWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Tari Mining"));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_rect);
        self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[1]);

        self.tari_amount.draw(f, v_chunks[2], state);

        self.button.draw(f, v_chunks[4], state);
    }
}