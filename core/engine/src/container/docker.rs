use super::{ContainerTask, DockerEvent};
use bollard::system::EventsOptions;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use tact::{ActorContext, Receiver};

impl ContainerTask {
    pub(super) fn subscribe_to_events(&mut self, ctx: &mut ActorContext<Self>) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["container".to_string()]);
        type_filter.insert(
            "container".to_string(),
            vec![self.container_info.container_name.clone()],
        );
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.docker.events(Some(opts)).map(DockerEvent::from);
        let receiver = Receiver::connect(stream, ctx.recipient());
        self.events = Some(receiver);
    }
}
