use super::actor::Actor;
use super::address::SendError;
use super::handler::Envelope;
use tokio::sync::{mpsc, watch};

pub enum ActorState {
    Active,
    Finished,
}

pub(crate) struct AddressJoint<A: Actor> {
    rx_event: mpsc::UnboundedReceiver<Envelope<A>>,
    tx_state: watch::Sender<ActorState>,
}

impl<A: Actor> AddressJoint<A> {
    pub fn new(
        rx_event: mpsc::UnboundedReceiver<Envelope<A>>,
        tx_state: watch::Sender<ActorState>,
    ) -> Self {
        Self { rx_event, tx_state }
    }

    pub async fn recv(&mut self) -> Option<Envelope<A>> {
        self.rx_event.recv().await
    }

    pub fn update_state(&mut self, state: ActorState) -> Result<(), SendError> {
        self.tx_state.send(state).map_err(|_| SendError)
    }
}
