use crate::recipient::Recipient;
use crate::task::Task;
use futures::{Stream, StreamExt};

pub struct Receiver {
    task: Task,
}

impl Receiver {
    pub fn connect<M, S>(stream: S, recipient: Recipient<M>) -> Self
    where
        M: Clone + Send + 'static,
        S: Stream<Item = M> + Send + 'static,
    {
        let task = Task::spawn(async move {
            tokio::pin!(stream);
            while let Some(event) = stream.next().await {
                recipient.send(event);
            }
        });
        Self { task }
    }
}
