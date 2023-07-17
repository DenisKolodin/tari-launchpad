use crate::container::{ContainerTask, ImageInfo};
use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use tact::{Actor, ActorContext, Do};

static DEFAULT_REGISTRY: &str = "quay.io/tarilabs";
static GRAFANA_REGISTRY: &str = "grafana";

pub struct Supervisor {}

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
    async fn handle(&mut self, _: ReadConfig, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        log::info!("Reading configuration...");
        Ok(())
    }
}

struct SpawnTasks;

#[async_trait]
impl Do<SpawnTasks> for Supervisor {
    async fn handle(&mut self, _: SpawnTasks, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let docker = Docker::connect_with_local_defaults()?;
        let info = ImageInfo::new(DEFAULT_REGISTRY, "tor", "latest");
        let tor_task = ContainerTask::new(docker.clone(), info);
        Ok(())
    }
}
