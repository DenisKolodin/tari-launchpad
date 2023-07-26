mod fsm;

use crate::docker::DockerEvent;
use crate::error::ParseError;
use crate::types::ManagedNetwork;
use anyhow::Error;
use async_trait::async_trait;
use bollard::models::{EventMessage, EventMessageTypeEnum};
use bollard::Docker;
use fsm::NetworkTaskFsm;
use tact::{Actor, ActorContext, Do, Receiver};
use thiserror::Error;

pub struct NetworkTask {
    docker: Docker,
    network_name: String,
    events: Option<Receiver>,
}

impl NetworkTask {
    pub fn new(scope: String, docker: Docker, mn: impl ManagedNetwork) -> Self {
        let network_name = format!("{}_{}", scope, mn.network_name());
        Self {
            docker,
            network_name,
            events: None,
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
        fsm.subscribe_to_events();
        // ctx.do_next(ProcessChanges)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),
    #[error("Can't parse the message: {0}")]
    ParseError(#[from] ParseError),
    #[error("Message for other network {actual}, but expected {expected}")]
    WrongNetwork { expected: String, actual: String },
}

#[async_trait]
impl Do<DockerEvent> for NetworkTask {
    type Error = EventError;

    async fn handle(
        &mut self,
        msg: DockerEvent,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        log::debug!("Event from {}: {msg:?}", self.network());
        let network_name = self.network();
        let mut event = None;
        let result = msg.result?;
        if let EventMessage {
            typ: Some(typ),
            action: Some(action),
            actor: Some(actor),
            ..
        } = result
        {
            if let Some(attributes) = actor.attributes {
                if let Some(name) = attributes.get("name") {
                    if network_name == *name {
                        if let EventMessageTypeEnum::NETWORK = typ {
                            let evt = Event::try_from(action)?;
                            event = Some(evt);
                        }
                    } else {
                        return Err(EventError::WrongNetwork {
                            expected: network_name.to_string(),
                            actual: name.to_string(),
                        });
                    }
                }
            }
        }
        if let Some(event) = event {
            let mut fsm = NetworkTaskFsm::new(self, ctx);
            // fsm.process_event(event)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Event {
    Destroyed,
    Created,
}

impl TryFrom<String> for Event {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Docker values!
        match value.as_ref() {
            "destroy" => Ok(Self::Destroyed),
            "create" => Ok(Self::Created),
            _ => Err(ParseError(value)),
        }
    }
}
