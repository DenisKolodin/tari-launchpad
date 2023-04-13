use super::actor::Actor;
use super::address::Address;
use super::context::ActorContext;
use super::handler::Envelope;
use tokio::sync::mpsc;

pub(super) struct ActorRuntime<A: Actor> {
    rx: mpsc::UnboundedReceiver<Envelope<A>>,
    actor: A,
    context: ActorContext<A>,
}

impl<A: Actor> ActorRuntime<A> {
    pub fn new(actor: A) -> Self {
        let (addr, rx) = Address::new();
        let context = ActorContext::new(addr);
        Self { rx, actor, context }
    }

    pub async fn entyrpoint(mut self) {
        while let Some(envelope) = self.rx.recv().await {
            let handler = envelope.into_handler();
            let res = handler.handle(&mut self.actor, &mut self.context).await;
            if let Err(err) = res {
                log::error!("Actor's handler failed: {err}");
            }
        }
    }

    pub fn context(&self) -> &ActorContext<A> {
        &self.context
    }
}
