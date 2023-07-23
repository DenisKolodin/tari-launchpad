use super::{ContainerTask, ContainerTaskFsm, DockerEvent};
use bollard::system::EventsOptions;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use tact::{ActorContext, Receiver};

impl<'a> ContainerTaskFsm<'a> {
    pub(super) fn subscribe_to_events(&mut self) {
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
}
