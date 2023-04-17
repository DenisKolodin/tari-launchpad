use super::actor::Actor;
use super::address::Address;
use super::context::ActorContext;
use super::handler::Envelope;
use std::any::type_name;
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
        let name = type_name::<Self>();
        let res = self.actor.initialize(&mut self.context).await;
        if let Err(err) = res {
            log::error!("Actor {name} can't be initialized: {err}");
        }
        while let Some(envelope) = self.rx.recv().await {
            let handler = envelope.into_handler();
            let res = handler.handle(&mut self.actor, &mut self.context).await;
            if let Err(err) = res {
                log::error!("Actor {name} handler failed: {err}");
            }
        }
        let res = self.actor.finalize(&mut self.context).await;
        if let Err(err) = res {
            log::error!("Actor {name} can't be finalized: {err}",);
        }
    }

    pub fn context(&self) -> &ActorContext<A> {
        &self.context
    }
}
