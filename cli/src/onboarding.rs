use crate::state::bus::Bus;
use crate::state::launchpad::LaunchpadAction;
use crate::state::onboarding::{Message, OnboardingAction, OnboardingDelta};
use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext, Do, Task};

pub struct OnboardingWorker {
    bus: Bus,
    actions: Option<Task>,
}

impl OnboardingWorker {
    pub fn new(bus: Bus) -> Self {
        Self { bus, actions: None }
    }
}

#[async_trait]
impl Actor for OnboardingWorker {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let task = self.bus.actions(ctx.recipient());
        self.actions = Some(task);
        Ok(())
    }
}

#[async_trait]
impl Do<OnboardingAction> for OnboardingWorker {
    async fn handle(
        &mut self,
        event: OnboardingAction,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        let msg = Message { text: MSG_1.into() };
        let delta = OnboardingDelta::Add(msg);
        self.bus.update(delta);
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
