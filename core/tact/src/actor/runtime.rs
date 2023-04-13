use super::context::ActorContext;
use super::handler::Actor;

pub struct ActorRuntime<A: Actor> {
    actor: A,
    context: ActorContext<A>,
}
