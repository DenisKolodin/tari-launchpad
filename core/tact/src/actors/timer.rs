use crate::actors::recipient::Notifier;
use crate::actors::utils::DropHandle;
use std::time::Duration;
use tokio::time;

pub struct Timer {
    handle: Option<DropHandle>,
}

impl Timer {
    pub fn new() -> Self {
        Self { handle: None }
    }

    pub fn timeout<M>(&mut self, duration: Duration, notifier: Notifier<M>)
    where
        M: Clone + Send + 'static,
    {
        let handle = tokio::spawn(async move {
            time::sleep(duration.into()).await;
            notifier.notify();
        });
        self.handle = Some(handle.into());
    }

    pub fn interval<M>(&mut self, duration: Duration, notifier: Notifier<M>)
    where
        M: Clone + Send + 'static,
    {
        let handle = tokio::spawn(async move {
            loop {
                time::sleep(duration.into()).await;
                notifier.notify();
            }
        });
        self.handle = Some(handle.into());
    }

    pub fn cancel(&mut self) {
        self.handle.take();
    }
}
