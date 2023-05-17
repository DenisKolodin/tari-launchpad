pub mod bus;
pub mod launchpad;
pub mod mining;
pub mod onboarding;

use crate::state::bus::Bus;
use launchpad::LaunchpadState;
use std::collections::VecDeque;
use tact::Recipient;

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
    pub events_queue: VecDeque<AppEvent>,
    pub bus: Bus,
    pub launchpad: LaunchpadState,
}

impl AppState {
    pub fn new(bus: Bus) -> Self {
        Self {
            focus_on: Focus::Onboarding,
            events_queue: VecDeque::new(),
            bus,
            launchpad: LaunchpadState::new(),
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
