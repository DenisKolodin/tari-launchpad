use super::actor::Actor;
use super::handler::Envelope;
use tokio::sync::mpsc;

pub enum ActorState {
    Active,
    Finished,
}

pub(crate) struct AddressJoint<A: Actor> {
    pub rx_event: mpsc::UnboundedReceiver<Envelope<A>>,
}

impl<A: Actor> AddressJoint<A> {
    pub fn new(rx_event: mpsc::UnboundedReceiver<Envelope<A>>) -> Self {
        Self { rx_event }
    }

    pub async fn recv(&mut self) -> Option<Envelope<A>> {
        self.rx_event.recv().await
    }
}
