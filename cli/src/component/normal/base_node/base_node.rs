use crate::component::elements::{block_with_title, logo};
use crate::component::normal::chrono_button::{ChronoButton, ChronoGetter};
use crate::component::{Component, ComponentEvent, Frame, Input, Pass};
use crate::state::{AppState, BaseNodeFocus, Focus};

use std::time::Duration;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

const LOGO: &str = r#"
╔╗ ┌─┐┌─┐┌─┐  ╔╗╔┌─┐┌┬┐┌─┐
╠╩╗├─┤└─┐├┤   ║║║│ │ ││├┤
╚═╝┴ ┴└─┘└─┘  ╝╚╝└─┘─┴┘└─┘
"#;

struct BaseNodeGetter;

impl ChronoGetter for BaseNodeGetter {
    fn get_duration(&self, _state: &AppState) -> Option<Duration> {
        None
    }

    fn get_label(&self, _state: &AppState) -> &str {
        if false {
            "Pause"
        } else {
            "Start node"
        }
    }
}

pub struct BaseNodeWidget {
    button: ChronoButton<BaseNodeGetter>,
}

impl BaseNodeWidget {
    pub fn new() -> Self {
        Self {
            button: ChronoButton::new(BaseNodeGetter),
        }
    }
}

impl Input for BaseNodeWidget {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == Focus::BaseNode(BaseNodeFocus::BaseNode) {
            match event.pass() {
                Pass::Up | Pass::Leave => {
                    state.focus_on(Focus::Root);
                }
                Pass::Enter | Pass::Space => {
                    // TODO: Toggle the base node state
                }
                _ => {}
            }
        }
    }
}

impl<B: Backend> Component<B> for BaseNodeWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let block = block_with_title(
            Some("Base Node"),
            state.focus_on == Focus::BaseNode(BaseNodeFocus::BaseNode),
        );
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
        // self.status_badge.draw(f, v_chunks[0], state);

        let logo = logo(LOGO);
        f.render_widget(logo, v_chunks[1]);

        // self.tari_amount.draw(f, v_chunks[2], state);

        self.button.draw(f, v_chunks[4], state);
    }
}
