use crate::network::{DockerEvent, NetworkTaskFsm};
use anyhow::Error;
use bollard::network::{CreateNetworkOptions, InspectNetworkOptions};
use bollard::system::EventsOptions;
use futures::StreamExt;
use std::collections::HashMap;
use tact::Receiver;

impl<'a> NetworkTaskFsm<'a> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["network".to_string()]);
        type_filter.insert("network".to_string(), vec![self.network().to_string()]);
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        // TODO: Use the filter map and converter instead
        let stream = self.docker.events(Some(opts)).map(DockerEvent::from);
        let recipient = self.ctx.recipient();
        let receiver = Receiver::connect(stream, recipient);
        self.task.events = Some(receiver);
    }

    pub async fn network_exists(&mut self) -> bool {
        let opts = InspectNetworkOptions {
            verbose: false,
            scope: "local",
        };
        self.docker
            .inspect_network(self.network(), Some(opts))
            .await
            .is_ok()
    }

    pub async fn try_create_network(&mut self) -> Result<(), Error> {
        let options = CreateNetworkOptions {
            name: self.network(),
            check_duplicate: true,
            driver: "bridge",
            internal: false,
            attachable: false,
            ingress: false,
            ipam: Default::default(),
            enable_ipv6: false,
            options: Default::default(),
            labels: Default::default(),
        };
        self.docker.create_network(options).await?;
        // TODO: Check warnings...
        Ok(())
    }

    pub async fn try_remove_network(&mut self) -> Result<(), Error> {
        self.docker.remove_network(self.network()).await?;
        Ok(())
    }
}
