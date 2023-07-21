use crate::container::ContainerTask;
use crate::images::{Tor, DEFAULT_REGISTRY};
use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use tact::{Actor, ActorContext, Do};

pub struct Supervisor {}

impl Supervisor {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Actor for Supervisor {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        ctx.do_next(ReadConfig)?;
        ctx.do_next(SpawnTasks)?;
        Ok(())
    }
}

struct ReadConfig;

#[async_trait]
impl Do<ReadConfig> for Supervisor {
    type Error = Error;

    async fn handle(
        &mut self,
        _: ReadConfig,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        log::info!("Reading configuration...");
        Ok(())
    }
}

struct SpawnTasks;

#[async_trait]
impl Do<SpawnTasks> for Supervisor {
    type Error = Error;

    async fn handle(
        &mut self,
        _: SpawnTasks,
        _ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let docker = Docker::connect_with_local_defaults()?;

        let tor_task = ContainerTask::new(docker.clone(), Tor);

        tor_task.start();
        Ok(())
    }
}
