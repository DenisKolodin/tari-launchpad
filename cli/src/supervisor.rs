use crate::dashboard::Dashboard;
use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext, Do};

pub struct Supervisor {}

impl Supervisor {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Actor for Supervisor {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let dashboard = Dashboard::new();
        let mut addr = dashboard.start();
        Ok(())
    }
}
