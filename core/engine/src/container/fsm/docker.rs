use crate::container::{ContainerTask, ContainerTaskFsm, DockerEvent, PullProgress, CreateImageOptions};
use crate::types::ContainerState;
use anyhow::Error;
use bollard::system::EventsOptions;
use bollard::container::{RemoveContainerOptions};
use bollard::image::{RemoveImageOptions};
use bollard::models::ContainerInspectResponse;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use tact::{ActorContext, Receiver};

impl<'a> ContainerTaskFsm<'a> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["container".to_string()]);
        type_filter.insert(
            "container".to_string(),
            vec![self.task.container_info.container_name.clone()],
        );
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.task.docker.events(Some(opts)).map(DockerEvent::from);
        let recipient = self.ctx.recipient();
        let receiver = Receiver::connect(stream, recipient);
        self.task.events = Some(receiver);
    }

    pub async fn image_exists(&mut self) -> bool {
        self.task.docker.inspect_image(self.task.image()).await.is_ok()
    }

    pub async fn container_state(&mut self) -> ContainerState {
        let res = self.task.docker.inspect_container(self.task.container(), None).await;
        // log::trace!("State of container {}: {:?}", self.inner.container_name, res);
        match res {
            Ok(ContainerInspectResponse { state: Some(state), .. }) => {
                if state.running.unwrap_or_default() {
                    ContainerState::Running
                } else {
                    ContainerState::NotRunning
                }
            },
            Ok(_) => ContainerState::NotRunning,
            Err(_) => ContainerState::NotFound,
        }
    }

    pub fn pull(&mut self) -> Receiver {
        let from_image = self.image().to_string();
        let opts = Some(CreateImageOptions {
            from_image,
            ..Default::default()
        });
        let stream = self
            .docker
            .create_image(opts, None, None)
            .map(PullProgress::from);
        Receiver::connect(stream, self.ctx.recipient())
    }

    pub async fn try_start_container(&mut self) -> Result<(), Error> {
        self.docker
            .start_container::<String>(self.container(), None)
            .await?;
        Ok(())
    }

    pub async fn try_kill_container(&mut self) -> Result<(), Error> {
        self.docker
            .kill_container::<String>(self.container(), None)
            .await?;
        Ok(())
    }

    pub async fn try_remove_container(&mut self) -> Result<(), Error> {
        let opts = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        self.docker
            .remove_container(self.container(), Some(opts))
            .await?;
        Ok(())
    }

    pub async fn try_remove_image(&mut self) -> Result<(), Error> {
        let image_name = self.image();
        let opts = Some(RemoveImageOptions {
            force: true,
            ..Default::default()
        });
        self.docker.remove_image(image_name, opts, None).await?;
        Ok(())
    }

}
