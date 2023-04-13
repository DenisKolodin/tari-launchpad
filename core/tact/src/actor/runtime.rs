use super::context::ActorContext;
use super::handler::{Actor, Envelope};
use tokio::sync::mpsc;

pub struct ActorRuntime<A: Actor> {
    rx: mpsc::UnboundedReceiver<Envelope<A>>,
    actor: A,
    context: ActorContext<A>,
}

impl<A: Actor> ActorRuntime<A> {
    async fn entyrpoint(mut self) {
        while let Some(envelope) = self.rx.recv().await {
            let handler = envelope.into_handler();
            let res = handler.handle(&mut self.actor, &mut self.context).await;
            if let Err(err) = res {
                log::error!("Actor's handler failed: {err}");
            }
        }
    }
}
