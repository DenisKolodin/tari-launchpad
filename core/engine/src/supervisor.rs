use anyhow::Error;
use async_trait::async_trait;
use tact::{Actor, ActorContext, Do};

pub struct Supervisor {}

#[async_trait]
impl Actor for Supervisor {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        ctx.do_next(ReadConfig)?;
        Ok(())
    }
}

struct ReadConfig;

#[async_trait]
impl Do<ReadConfig> for Supervisor {
    async fn handle(&mut self, _: ReadConfig, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        log::info!("Reading configuration...");
        Ok(())
    }
}
