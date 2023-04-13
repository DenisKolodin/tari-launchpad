use super::actor::Actor;
use super::address::Address;

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
}
