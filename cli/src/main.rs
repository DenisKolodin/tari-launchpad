use anyhow::Error;
use tact::Actor;
use tari_lp_cli::supervisor::Supervisor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let supervisor = Supervisor::new();
    let mut addr = supervisor.start();
    addr.join().await?;
    Ok(())
}
