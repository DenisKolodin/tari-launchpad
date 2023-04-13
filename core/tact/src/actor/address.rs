use super::handler::{Actor, Envelope, OnEvent};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
#[error("Can't send an event to an actor")]
pub struct SendError;

#[derive(Debug, Clone)]
pub struct Address<A: Actor> {
    tx: mpsc::UnboundedSender<Envelope<A>>,
}

impl<A: Actor> Address<A> {
    pub fn send<E>(&self, event: E) -> Result<(), SendError>
    where
        A: OnEvent<E>,
        E: Send + 'static,
    {
        let envelope = Envelope::from_event(event);
        self.tx.send(envelope).map_err(|_| SendError)
    }
}
