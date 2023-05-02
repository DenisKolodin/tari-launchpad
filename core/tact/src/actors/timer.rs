use tokio::task::JoinHandle;

pub struct Timer {
    handle: Option<JoinHandle<()>>,
}

impl Timer {
    pub fn timeout(&mut self) {}
}
