use crate::actors::utils::DropHandle;
use futures::Future;

pub struct Task {
    handle: DropHandle,
}

impl Task {
    pub fn spawn<T>(fut: T) -> Self
    where
        T: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(fut);
        Self {
            handle: DropHandle::from(handle),
        }
    }
}
