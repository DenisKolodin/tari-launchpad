mod amount;
mod chrono_button;
mod merged_mining;
mod status_badge;
mod tari_mining;
mod tip;

use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::{AppState, FocusOn};
use crossterm::event::{KeyCode, KeyEvent};
use merged_mining::MergedMiningWidget;
use tari_mining::TariMiningWidget;
use tip::MiningTip;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct MiningScene {
    mining_tip: MiningTip,
    tari_mining: TariMiningWidget,
    merged_mining: MergedMiningWidget,
}

impl MiningScene {
    pub fn new() -> Self {
        Self {
            mining_tip: MiningTip::new(),
            tari_mining: TariMiningWidget::new(),
            merged_mining: MergedMiningWidget::new(),
        }
    }
}

impl Input for MiningScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        match event {
            ComponentEvent::Focus => {
                state.focus_on(FocusOn::TariMining);
            }
            _ => {}
        }
        if state.focus_on == FocusOn::TariMining {
            if let ComponentEvent::Key(key) = event {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.focus_on(FocusOn::Root);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl<B: Backend> Component<B> for MiningScene {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Percentage(50),
            Constraint::Min(0),
        ];
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.mining_tip.draw(f, v_chunks[0], state);

        let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(v_chunks[1]);
        self.tari_mining.draw(f, h_chunks[0], state);
        self.merged_mining.draw(f, h_chunks[1], state);
        // let block = block_with_title(None);
        // f.render_widget(block, rect);
    }
}
