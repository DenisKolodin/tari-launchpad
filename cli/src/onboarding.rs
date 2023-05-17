use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext};

pub struct OnboardingWorker {}

#[async_trait]
impl Actor for OnboardingWorker {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        Ok(())
    }
}
