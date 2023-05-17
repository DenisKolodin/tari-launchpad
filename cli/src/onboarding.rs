use crate::state::bus::Bus;
use crate::state::launchpad::LaunchpadAction;
use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext, Do};

pub struct OnboardingWorker {
    bus: Bus,
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
