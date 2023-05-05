use crate::actors::recipient::Notifier;
use crate::actors::task::Task;
use std::time::Duration;
use tokio::time;

pub struct Timeout {
    task: Task,
}

impl Timeout {
    pub fn spawn<M>(duration: Duration, notifier: Notifier<M>) -> Self
    where
        M: Clone + Send + 'static,
    {
        let task = Task::spawn(async move {
            time::sleep(duration.into()).await;
            notifier.notify();
        });
        Self { task }
    }
}

pub struct Interval {
    task: Task,
}

impl Interval {
    pub fn spawn<M>(duration: Duration, notifier: Notifier<M>) -> Self
    where
        M: Clone + Send + 'static,
    {
        let task = Task::spawn(async move {
            loop {
                time::sleep(duration.into()).await;
                notifier.notify();
            }
        });
        Self { task }
    }
}
