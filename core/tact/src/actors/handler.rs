use super::actor::Actor;
use super::context::ActorContext;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Do<E>: Actor {
    async fn handle(&mut self, event: E, ctx: &mut ActorContext<Self>) -> Result<(), Error>;
}

pub struct Envelope<A: Actor> {
    handler: Box<dyn Handler<A>>,
}

impl<A: Actor> Envelope<A> {
    pub(crate) fn into_handler(self) -> Box<dyn Handler<A>> {
        self.handler
    }

    pub fn from_event<E>(event: E) -> Self
    where
        A: Do<E>,
        E: Send + 'static,
    {
        let handler = HandlerImpl { event };
        Self {
            handler: Box::new(handler),
        }
    }
}

#[async_trait]
pub(crate) trait Handler<A: Actor>: Send {
    async fn handle(self: Box<Self>, actor: &mut A, ctx: &mut ActorContext<A>)
        -> Result<(), Error>;
}

struct HandlerImpl<E> {
    event: E,
}

#[async_trait]
impl<A: Do<E>, E: Send> Handler<A> for HandlerImpl<E> {
    async fn handle(
        self: Box<Self>,
        actor: &mut A,
        ctx: &mut ActorContext<A>,
    ) -> Result<(), Error> {
        actor.handle(self.event, ctx).await
    }
}
