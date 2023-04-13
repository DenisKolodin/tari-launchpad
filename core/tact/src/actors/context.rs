use super::actor::Actor;
use super::address::{Address, SendError};
use super::action::Do;

pub struct ActorContext<A: Actor> {
    address: Address<A>,
}

impl<A: Actor> ActorContext<A> {
    pub(super) fn new(address: Address<A>) -> Self {
        Self { address }
    }

    pub fn address(&self) -> &Address<A> {
        &self.address
    }

    pub fn do_next<E>(&self, action: E) -> Result<(), SendError>
    where
        A: Do<E>,
        E: Send + 'static,
    {
        self.address.send(action)
    }
}
