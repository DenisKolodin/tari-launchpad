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

    pub fn send<M>(&mut self, action: M)
    where
        LaunchpadAction: From<M>,
    {
        self.actions.send(action.into()).ok();
    }

    pub fn update<M>(&mut self, delta: M)
    where
        LaunchpadDelta: From<M>,
    {
        self.state
            .send_modify(move |state| state.update(delta.into()));
    }

    pub fn subscribe<M>(&mut self, recipient: Recipient<M>) -> Task
    where
        Option<M>: From<LaunchpadAction>,
        M: 'static,
    {
        let mut rx = self.actions.subscribe();
        Task::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Some(event) = msg.into() {
                    if let Err(_err) = recipient.send(event) {
                        // TODO: log error
                        break;
                    }
                }
            }
        })
    }
}
