use crossterm::event::KeyEvent;
use rust_decimal::Decimal;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Focus {
    Root,
    TariMining,
    MergedMining,
}

impl Default for Focus {
    fn default() -> Self {
        Self::Root
    }
}

pub enum AppEvent {
    SetFocus(Focus),
}

pub struct AppState {
    pub focus_on: Focus,
    pub tari_mining: TariMiningInfo,
    pub merged_mining: MergedMiningInfo,
    pub events_queue: VecDeque<AppEvent>,
}

impl AppState {
    pub fn new() -> Self {
        let tari_mining = TariMiningInfo {
            is_active: true,
            tari_amount: 123_456.into(),
        };
        let merged_mining = MergedMiningInfo {
            is_active: false,
            tari_amount: 45_000.into(),
            monero_amount: Decimal::new(35, 1),
        };
        Self {
            focus_on: Focus::default(),
            tari_mining,
            merged_mining,
            events_queue: VecDeque::new(),
        }
    }

    pub fn focus_on(&mut self, value: Focus) {
        let event = AppEvent::SetFocus(value);
        self.events_queue.push_front(event);
    }

    pub fn process_events(&mut self) -> bool {
        if self.events_queue.is_empty() {
            false
        } else {
            for event in self.events_queue.drain(..) {
                match event {
                    AppEvent::SetFocus(value) => {
                        self.focus_on = value;
                    }
                }
            }
            true
        }
    }
}

pub struct TariMiningInfo {
    pub is_active: bool,
    pub tari_amount: Decimal,
}

pub struct MergedMiningInfo {
    pub is_active: bool,
    pub tari_amount: Decimal,
    pub monero_amount: Decimal,
}
