use anyhow::Error;
use tact::actors::Actor;
use tari_lp_cli::dashboard::Dashboard;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let dashboard = Dashboard::new();
    let mut addr = dashboard.start();
    tokio::signal::ctrl_c().await?;
    addr.interrupt()?;
    Ok(())
}
