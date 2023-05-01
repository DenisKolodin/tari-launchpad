use crate::component::elements::{block_with_title, logo};
use crate::component::normal::mining::amount::{AmountGetter, AmountIndicator};
use crate::component::normal::mining::chrono_button::ChronoButton;
use crate::component::normal::mining::status_badge::{StatusBadge, StatusGetter};
use crate::component::{Component, ComponentEvent, Frame, Input, Pass};
use crate::state::{AppState, Focus};
use crossterm::event::KeyEvent;
use rust_decimal::Decimal;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;

const LOGO: &str = r#"
╔╦╗┌─┐┬─┐┌─┐┌─┐┌┬┐  ╔╦╗┬┌┐┌┬┌┐┌┌─┐
║║║├┤ ├┬┘│ ┬├┤  ││  ║║║│││││││││ ┬
╩ ╩└─┘┴└─└─┘└─┘─┴┘  ╩ ╩┴┘└┘┴┘└┘└─┘
"#;

struct MergedMiningGetter;

impl StatusGetter for MergedMiningGetter {
    fn get_status(&self, state: &AppState) -> (&str, Color) {
        if state.merged_mining.is_active {
            ("(Running)", Color::Green)
        } else {
            ("(Ready to set)", Color::Cyan)
        }
    }
}

struct XtrGetter;

impl AmountGetter for XtrGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str) {
        let amount = state.merged_mining.tari_amount;
        (amount, "XTR")
    }
}

struct XmrGetter;

impl AmountGetter for XmrGetter {
    fn get_amount(&self, state: &AppState) -> (Decimal, &str) {
        let amount = state.merged_mining.monero_amount;
        (amount, "XMR")
    }
}

pub struct MergedMiningWidget {
    status_badge: StatusBadge<MergedMiningGetter>,
    tari_amount: AmountIndicator<XtrGetter>,
    monero_amount: AmountIndicator<XmrGetter>,
    button: ChronoButton,
}

impl MergedMiningWidget {
    pub fn new() -> Self {
        Self {
            status_badge: StatusBadge::new(MergedMiningGetter),
            tari_amount: AmountIndicator::new(XtrGetter),
            monero_amount: AmountIndicator::new(XmrGetter),
            button: ChronoButton::new(),
        }
    }
}

impl Input for MergedMiningWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == Focus::MergedMining {
            match event.pass() {
                Pass::Left => {
                    state.focus_on(Focus::TariMining);
                }
                Pass::Up | Pass::Out => {
                    state.focus_on(Focus::Root);
                }
                _ => {}
            }
        }
    }
}

impl<B: Backend> Component<B> for MergedMiningWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(Some("Merged Mining"), state.focus_on == Focus::MergedMining);
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let constraints = [
            Constraint::Length(1),
            Constraint::Length(3),
            // Constraint::Percentage(50),
            Constraint::Length(1),
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
        self.monero_amount.draw(f, v_chunks[3], state);

        self.button.draw(f, v_chunks[5], state);
    }
}
