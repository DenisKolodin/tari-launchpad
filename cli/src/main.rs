use anyhow::Error;
use tari_sdm::SdmScope;
use tari_sdm_launchpad::{config::LaunchpadConfig, files::Configurator, images, networks, volumes};
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let mut scope = SdmScope::connect("esmeralda")?;

    scope.add_network(networks::LocalNet::default()).await?;
    scope.add_volume(volumes::SharedVolume::default()).await?;
    scope.add_image(images::Tor::default()).await?;
    scope.add_image(images::TariBaseNode::default()).await?;
    scope.add_image(images::TariWallet::default()).await?;
    // scope.add_image(images::Loki::default()).await?;
    // scope.add_image(images::Promtail::default()).await?;
    // scope.add_image(images::Grafana::default()).await?;

    ctrl_c().await?;
    log::info!("Set config");

    let mut configurator = Configurator::init()?;
    let data_directory = configurator.base_path().clone();
    configurator.repair_configuration().await?;
    let mut config = LaunchpadConfig {
        data_directory,
        with_monitoring: true,
        tor_control_password: create_password(16).into(),
        ..Default::default()
    };
    scope.set_config(Some(config.clone())).await?;

    ctrl_c().await?;
    log::info!("Turn off monitoring");
    config.with_monitoring = false;
    scope.set_config(Some(config)).await?;

    ctrl_c().await?;
    log::info!("Reset config");
    scope.set_config(None).await?;

    ctrl_c().await?;
    scope.stop();
    // TODO: Get events from the manager
    Ok(())
}

/// Create a cryptographically secure password on length `len`
pub fn create_password(len: usize) -> String {
    use rand::distributions::{Alphanumeric, Distribution};
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_iter(&mut rng).take(len).map(char::from).collect()
}
