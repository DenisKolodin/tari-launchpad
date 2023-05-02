use crate::actors::recipient::Notifier;
use derive_more::From;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;

#[derive(From)]
struct DropHandle(JoinHandle<()>);

impl Drop for DropHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}

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
