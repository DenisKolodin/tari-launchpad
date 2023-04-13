use super::context::ActorContext;
use anyhow::Error;
use async_trait::async_trait;

pub trait Actor: Send + Sized {}

#[async_trait]
pub trait OnEvent<E>: Actor {
    async fn handle(&mut self, event: E, ctx: &mut ActorContext<Self>);
}

pub struct Envelope<A: Actor> {
    handler: Box<dyn Handler<A>>,
}

#[async_trait]
pub trait Handler<A: Actor>: Send {
    async fn handle(self: Box<Self>, actor: &mut A, ctx: &mut ActorContext<A>);
}

pub struct HandlerImpl<E> {
    event: E,
}

#[async_trait]
impl<A: OnEvent<E>, E: Send> Handler<A> for HandlerImpl<E> {
    async fn handle(self: Box<Self>, actor: &mut A, ctx: &mut ActorContext<A>) {
        actor.handle(self.event, ctx).await
    }
}
