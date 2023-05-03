use derive_more::From;
use tokio::task::JoinHandle;

#[derive(From)]
pub struct DropHandle(JoinHandle<()>);

impl Drop for DropHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
