use super::action::{Do, Interrupt};
use super::actor::Actor;
use super::handler::Envelope;
use super::joint::AddressJoint;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
#[error("Can't send an event to an actor")]
pub struct SendError;

pub struct Address<A: Actor> {
    tx_event: mpsc::UnboundedSender<Envelope<A>>,
}

impl<A: Actor> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            tx_event: self.tx_event.clone(),
        }
    }
}

impl<A: Actor> Address<A> {
    pub(super) fn new() -> (Self, AddressJoint<A>) {
        let (tx_event, rx_event) = mpsc::unbounded_channel();
        let joint = AddressJoint::new(rx_event);
        (Self { tx_event }, joint)
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
