use super::launchpad::{LaunchpadAction, LaunchpadDelta, LaunchpadState};
use std::sync::Arc;
use tokio::sync::{broadcast, watch};

#[derive(Debug, Clone)]
pub struct Bus {
    state: Arc<watch::Sender<LaunchpadState>>,
    actions: broadcast::Sender<LaunchpadAction>,
}

impl Bus {
    pub fn update(&self, delta: LaunchpadDelta) {
        self.state.send_modify(move |state| state.update(delta));
    }

    // TODO: Add `subscribe` method
}
