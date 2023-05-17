use crate::actor::Actor;
use crate::context::ActorContext;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Do<E>: Actor {
    async fn handle(&mut self, action: E, ctx: &mut ActorContext<Self>) -> Result<(), Error>;
}

pub struct Interrupt;
