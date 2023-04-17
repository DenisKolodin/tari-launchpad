use super::action::{Do, Interrupt};
use super::actor::Actor;
use super::handler::Envelope;
use super::joint::{ActorState, AddressJoint};
use thiserror::Error;
use tokio::sync::{mpsc, watch};

#[derive(Debug, Error)]
#[error("Can't send an event to an actor")]
pub struct SendError;

pub struct Address<A: Actor> {
    tx_event: mpsc::UnboundedSender<Envelope<A>>,
    rx_state: watch::Receiver<ActorState>,
}

impl<A: Actor> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            tx_event: self.tx_event.clone(),
            rx_state: self.rx_state.clone(),
        }
    }
}

impl<A: Actor> Address<A> {
    pub(super) fn new() -> (Self, AddressJoint<A>) {
        let (tx_event, rx_event) = mpsc::unbounded_channel();
        let (tx_state, rx_state) = watch::channel(ActorState::Active);
        let joint = AddressJoint::new(rx_event, tx_state);
        let addr = Self { tx_event, rx_state };
        (addr, joint)
    }

    pub fn send<E>(&self, event: E) -> Result<(), SendError>
    where
        A: Do<E>,
        E: Send + 'static,
    {
        let envelope = Envelope::from_event(event);
        self.tx_event.send(envelope).map_err(|_| SendError)
    }

    pub fn interrupt(&mut self) -> Result<(), SendError>
    where
        A: Do<Interrupt>,
    {
        self.send(Interrupt)
    }
}
