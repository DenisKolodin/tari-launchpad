use tari_launchpad_protocol::launchpad::{LaunchpadState, Reaction};

use crate::widget::{SharedState, State};

pub static REMOTE_STATE: SharedState<RemoteState> = SharedState::new();

#[derive(Default, Debug)]
pub struct RemoteState {
    pub state: LaunchpadState,
    pub loaded: bool,
}

impl State for RemoteState {
    type Delta = Reaction;

    fn apply(&mut self, delta: Self::Delta) {
        match delta {
            Reaction::State(state) => {
                self.state = state;
            },
            Reaction::Delta(delta) => {
                self.state.apply(delta);
            },
        }
    }
}
