use crate::actors::utils::DropHandle;
use crate::actors::Recipient;
use futures::{Stream, StreamExt};

pub struct Receiver {
    handle: Option<DropHandle>,
}

impl Receiver {
    pub fn connect<M, S>(&mut self, stream: S, recipient: Recipient<M>)
    where
        M: Clone + Send + 'static,
        S: Stream<Item = M> + Send + 'static,
    {
        let handle = tokio::spawn(async move {
            tokio::pin!(stream);
            while let Some(event) = stream.next().await {
                recipient.send(event);
            }
        });
        self.handle = Some(handle.into());
    }

    pub fn cancel(&mut self) {
        self.handle.take();
    }
}
