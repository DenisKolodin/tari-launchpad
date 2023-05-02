use crate::actors::recipient::Notifier;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;

pub struct Timer {
    handle: Option<JoinHandle<()>>,
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.cancel();
    }
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
        self.handle = Some(handle);
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
        self.handle = Some(handle);
    }

    pub fn cancel(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}
