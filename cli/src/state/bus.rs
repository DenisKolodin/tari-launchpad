use super::launchpad::{LaunchpadAction, LaunchpadDelta, LaunchpadState};
use std::sync::Arc;
use tact::actors::{Recipient, Task};
use tokio::sync::watch::Ref;
use tokio::sync::{broadcast, watch};

#[derive(Debug, Clone)]
pub struct Bus {
    state: Arc<watch::Sender<LaunchpadState>>,
    actions: broadcast::Sender<LaunchpadAction>,
}

impl Bus {
    pub fn new() -> Self {
        let state = LaunchpadState::new();
        let (state_tx, state_rx) = watch::channel(state);
        let (actions_tx, actions_rx) = broadcast::channel(64);
        Self {
            state: Arc::new(state_tx),
            actions: actions_tx,
        }
    }

    pub fn state(&self) -> Ref<'_, LaunchpadState> {
        self.state.borrow()
    }

    pub fn update(&mut self, delta: LaunchpadDelta) {
        self.state.send_modify(move |state| state.update(delta));
    }

    pub fn subscribe<M>(&mut self, recipient: Recipient<M>)
    where
        Option<M>: From<LaunchpadAction>,
        M: 'static,
    {
        let mut rx = self.actions.subscribe();
        let task = Task::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Some(event) = msg.into() {
                    if let Err(_err) = recipient.send(event) {
                        // TODO: log error
                        break;
                    }
                }
            }
        });
    }
}
