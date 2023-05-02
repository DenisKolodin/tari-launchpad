use crate::actors::action::Do;
use crate::actors::address::{Address, SendError};

pub trait Sender<M>: Send {
    fn send(&self, msg: M) -> Result<(), SendError>;
}

impl<A, M> Sender<M> for Address<A>
where
    A: Do<M>,
    M: Send + 'static,
{
    fn send(&self, msg: M) -> Result<(), SendError> {
        Address::send(self, msg)
    }
}

pub struct Recipient<M> {
    sender: Box<dyn Sender<M>>,
}

impl<M> Recipient<M> {
    pub fn send(&self, msg: M) -> Result<(), SendError> {
        self.sender.send(msg)
    }
}

pub struct Notifier<M> {
    message: M,
    sender: Box<dyn Sender<M>>,
}

impl<M: Clone> Notifier<M> {
    pub fn notify(&self) -> Result<(), SendError> {
        let msg = self.message.clone();
        self.sender.send(msg)
    }
}
