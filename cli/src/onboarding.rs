use crate::state::bus::Bus;
use crate::state::launchpad::LaunchpadAction;
use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext, Do};

pub struct OnboardingWorker {
    bus: Bus,
}

impl OnboardingWorker {
    pub fn new(bus: Bus) -> Self {
        Self { bus }
    }
}

#[async_trait]
impl Actor for OnboardingWorker {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        self.bus.subscribe(ctx.recipient());
        Ok(())
    }
}

#[async_trait]
impl Do<LaunchpadAction> for OnboardingWorker {
    async fn handle(
        &mut self,
        event: LaunchpadAction,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

const MSG_1: &str = "
Hi! My name is T-Bot. It is a great pleasure and an honor to meet you!
I have no memory of human faces, so if our paths have already crossed in the Aurora app, I’m glad to see you again!
";

const MSG_2: &str = "
I'm kind of like Gandalf, Dumbledore or Obi-Wan Kenobi. You know, the guy who makes sure the novice gets to a certain destination. Spoiler alert: in this saga the guide will survive. Regardless of whether this is your first contact with cryptocurrencies or you are advanced in it, I will stay with you until the Tari Launchpad setup process is successfully completed.
";

const MSG_3: &str = "
So let's get started! 🚀 The setup process usually takes 5 to 10 minutes. A duo like you and me should be able to deal with it quickly, right?
";
