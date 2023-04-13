use super::address::Address;
use super::context::ActorContext;
use super::runtime::ActorRuntime;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Actor: Send + Sized + 'static {
    async fn initialize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn start(self) -> Address<Self> {
        let runtime = ActorRuntime::new(self);
        let address = runtime.context().address().clone();
        tokio::spawn(runtime.entyrpoint());
        address
    }
}
