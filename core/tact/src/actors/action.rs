use super::actor::Actor;
use super::context::ActorContext;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Do<E>: Actor {
    async fn handle(&mut self, action: E, ctx: &mut ActorContext<Self>) -> Result<(), Error>;
}
