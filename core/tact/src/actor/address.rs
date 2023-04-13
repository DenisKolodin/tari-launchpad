use super::handler::{Actor, Envelope};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Address<A: Actor> {
    tx: mpsc::UnboundedSender<Envelope<A>>,
}
