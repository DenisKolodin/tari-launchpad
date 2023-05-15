mod amount;
mod chrono_button;
mod merged_mining;
mod status_badge;
mod tari_mining;

use crate::component::{Component, ComponentEvent, Frame, Input};
use crate::state::AppState;

use super::hint::HintLine;
use merged_mining::MergedMiningWidget;
use tari_mining::TariMiningWidget;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct MiningScene {
    hint: HintLine,
    tari_mining: TariMiningWidget,
    merged_mining: MergedMiningWidget,
}

impl MiningScene {
    pub fn new() -> Self {
        Self {
            hint: HintLine::new(),
            tari_mining: TariMiningWidget::new(),
            merged_mining: MergedMiningWidget::new(),
        }
    }
}

impl Input for MiningScene {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.tari_mining.on_event(event, state);
        self.merged_mining.on_event(event, state);
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
        self.hint.draw(f, v_chunks[0], state);

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
