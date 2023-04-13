use super::actor::Actor;
use super::handler::{Envelope, OnEvent};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
#[error("Can't send an event to an actor")]
pub struct SendError;

pub struct Address<A: Actor> {
    tx: mpsc::UnboundedSender<Envelope<A>>,
}

impl<A: Actor> Clone for Address<A> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<A: Actor> Address<A> {
    pub(super) fn new() -> (Self, mpsc::UnboundedReceiver<Envelope<A>>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, rx)
    }

    pub fn send<E>(&self, event: E) -> Result<(), SendError>
    where
        A: OnEvent<E>,
        E: Send + 'static,
    {
        let envelope = Envelope::from_event(event);
        self.tx.send(envelope).map_err(|_| SendError)
    }
}
