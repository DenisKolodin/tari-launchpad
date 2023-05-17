pub mod mining;
pub mod onboarding;

use mining::{MergedMiningInfo, TariMiningInfo};
use onboarding::Onboarding;
use rust_decimal::Decimal;
use std::collections::VecDeque;
use tact::actors::Recipient;

#[derive(Debug, Clone)]
pub enum StateAction {
    Redraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Focus {
    Onboarding,
    Root,
    Mining(MiningFocus),
    BaseNode(BaseNodeFocus),
    Wallet(WalletFocus),
}

impl Focus {
    pub fn is_onboarding(&self) -> bool {
        matches!(self, Self::Onboarding)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MiningFocus {
    TariMining,
    MergedMining,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseNodeFocus {
    BaseNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalletFocus {
    Password,
}

pub enum AppEvent {
    SetFocus(Focus),
    Redraw,
}

pub struct AppState {
    pub focus_on: Focus,
    pub tari_mining: TariMiningInfo,
    pub merged_mining: MergedMiningInfo,
    pub events_queue: VecDeque<AppEvent>,
    pub onboarding: Onboarding,
    // pub recipient: Recipient<StateAction>,
}

impl AppState {
    pub fn new(recipient: Recipient<StateAction>) -> Self {
        let tari_mining = TariMiningInfo {
            mining_started: None,
            tari_amount: 123_456.into(),
        };
        let merged_mining = MergedMiningInfo {
            mining_started: None,
            tari_amount: 45_000.into(),
            monero_amount: Decimal::new(35, 1),
        };
        Self {
            focus_on: Focus::Onboarding,
            tari_mining,
            merged_mining,
            events_queue: VecDeque::new(),
            onboarding: Onboarding::default(),
            // recipient,
        }
    }

    pub fn focus_on(&mut self, value: Focus) {
        let event = AppEvent::SetFocus(value);
        self.events_queue.push_front(event);
    }

    pub fn redraw(&mut self) {
        let event = AppEvent::Redraw;
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
                    AppEvent::Redraw => {}
                }
            }
            true
        }
    }
}
