use super::address::Address;
use super::runtime::ActorRuntime;
use super::handler::Do;
use super::action::Init;

pub trait Actor: Send + Sized + 'static {
    fn start(self) -> Address<Self>
    where
        Self: Do<Init>,
    {
        let runtime = ActorRuntime::new(self);
        let address = runtime.context().address().clone();
        tokio::spawn(runtime.entyrpoint());
        address
    }
}
