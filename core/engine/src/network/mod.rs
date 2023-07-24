mod fsm;

use crate::types::ManagedNetwork;
use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use fsm::NetworkTaskFsm;
use tact::{Actor, ActorContext};

pub struct NetworkTask {
    docker: Docker,
    network_name: String,
}

impl NetworkTask {
    pub fn new(scope: String, docker: Docker, mn: impl ManagedNetwork) -> Self {
        let network_name = format!("{}_{}", scope, mn.network_name());
        Self {
            docker,
            network_name,
        }
    }

    fn network(&self) -> &str {
        &self.network_name
    }
}

#[async_trait]
impl Actor for NetworkTask {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        log::info!("Spawning a task to control the network: {}", self.network());
        let mut fsm = NetworkTaskFsm::new(self, ctx);
        // TODO: fsm.subscribe_to_events();
        // ctx.do_next(ProcessChanges)?;
        Ok(())
    }
}
