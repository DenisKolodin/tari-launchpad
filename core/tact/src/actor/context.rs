use super::address::Address;
use super::handler::Actor;

pub struct ActorContext<A: Actor> {
    address: Address<A>,
}
