use super::address::Address;
use super::runtime::ActorRuntime;

pub trait Actor: Send + Sized + 'static {
    fn start(self) -> Address<Self> {
        let runtime = ActorRuntime::new(self);
        let address = runtime.context().address().clone();
        tokio::spawn(runtime.entyrpoint());
        address
    }
}
