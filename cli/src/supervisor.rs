use crate::dashboard::Dashboard;
use crate::onboarding::OnboardingWorker;
use crate::state::bus::Bus;
use anyhow::Error;
use async_trait::async_trait;
use tact::actors::{Actor, ActorContext, Do};

pub struct Supervisor {
    bus: Option<Bus>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self { bus: None }
    }
}

#[async_trait]
impl Actor for Supervisor {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let bus = Bus::new();
        let dashboard = Dashboard::new();
        let mut addr = dashboard.start();
        let onboarding = OnboardingWorker::new(bus.clone());
        let mut addr = onboarding.start();
        self.bus = Some(bus);
        Ok(())
    }
}
