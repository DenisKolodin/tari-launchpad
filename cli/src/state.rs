use rust_decimal::Decimal;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

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
    pub mining_started: Option<Instant>,
    pub tari_amount: Decimal,
}

impl TariMiningInfo {
    pub fn is_active(&self) -> bool {
        self.mining_started.is_some()
    }

    pub fn mining_duration(&self) -> Option<Duration> {
        self.mining_started.as_ref().map(Instant::elapsed)
    }

    pub fn toggle(&mut self) {
        if self.mining_started.is_some() {
            self.mining_started = None;
        } else {
            self.mining_started = Some(Instant::now());
        }
    }
}

pub struct MergedMiningInfo {
    pub mining_started: Option<Instant>,
    pub tari_amount: Decimal,
    pub monero_amount: Decimal,
}

impl MergedMiningInfo {
    pub fn is_active(&self) -> bool {
        self.mining_started.is_some()
    }

    pub fn mining_duration(&self) -> Option<Duration> {
        self.mining_started.as_ref().map(Instant::elapsed)
    }

    pub fn toggle(&mut self) {
        if self.mining_started.is_some() {
            self.mining_started = None;
        } else {
            self.mining_started = Some(Instant::now());
        }
    }
}
