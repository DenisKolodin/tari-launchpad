use crate::types::{Args, Envs, ManagedContainer, Mounts, Networks, Ports, Volumes};
use anyhow::{anyhow as err, Error};
use async_trait::async_trait;
use bollard::container::{Config, CreateContainerOptions, RemoveContainerOptions};
use bollard::errors::Error as BollardError;
use bollard::image::CreateImageOptions;
use bollard::models::{CreateImageInfo, EventMessage, HostConfig};
use bollard::system::EventsOptions;
use bollard::Docker;
use derive_more::From;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tact::{Actor, ActorContext, Do, Receiver, Recipient, Timeout};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ImageInfo {
    scope: String,
    registry: String,
    image_name: String,
    tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContainerInfo {
    image_info: ImageInfo,
    /// The full image name
    image_name: String,
    container_name: String,
}

impl From<ImageInfo> for ContainerInfo {
    fn from(image_info: ImageInfo) -> ContainerInfo {
        let image_name = format!(
            "{}/{}:{}",
            image_info.registry, image_info.image_name, image_info.tag
        );
        let container_name = format!("{}_{}", image_info.scope, image_info.image_name);
        Self {
            image_info,
            image_name,
            container_name,
        }
    }
}

#[derive(Debug)]
enum ContainerState {
    Idle,
    PullingImage(Receiver),
    StartingContainer,
    WaitContainerStart(Timeout),
    KillingContainer,
    WatiContainerKill(Timeout),
}

pub struct ContainerTask {
    docker: Docker,
    mc: Box<dyn ManagedContainer>,
    container_info: ContainerInfo,
    state: ContainerState,
    pull_progress: u8,
    events: Option<Receiver>,
}

impl ContainerTask {
    pub fn new(scope: String, docker: Docker, mc: impl ManagedContainer) -> Self {
        let image_info = ImageInfo {
            scope,
            registry: mc.registry().to_string(),
            image_name: mc.image_name().to_string(),
            tag: mc.tag().to_string(),
        };
        let container_info = ContainerInfo::from(image_info);
        Self {
            docker,
            mc: Box::new(mc),
            container_info,
            state: ContainerState::Idle,
            pull_progress: 0,
            events: None,
        }
    }
}

impl ContainerTask {
    fn subscribe_to_container_events(&mut self, ctx: &mut ActorContext<Self>) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["container".to_string()]);
        type_filter.insert(
            "container".to_string(),
            vec![self.container_info.container_name.clone()],
        );
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.docker.events(Some(opts)).map(DockerEvent::from);
        let receiver = Receiver::connect(stream, ctx.recipient());
        self.events = Some(receiver);
    }
}

#[async_trait]
impl Actor for ContainerTask {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        log::info!(
            "Spawning a task to control: {}",
            self.container_info.image_name
        );
        self.subscribe_to_container_events(ctx);
        ctx.do_next(CheckImage)?;
        Ok(())
    }
}

#[derive(Debug, From)]
struct DockerEvent {
    result: Result<EventMessage, BollardError>,
}

#[async_trait]
impl Do<DockerEvent> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        msg: DockerEvent,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let name = &self.container_info.container_name;
        log::debug!("Event from {name}: {msg:?}");
        Ok(())
    }
}

struct CheckImage;

#[async_trait]
impl Do<CheckImage> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        _: CheckImage,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let exist = self
            .docker
            .inspect_image(&self.container_info.image_name)
            .await
            .is_ok();
        if !exist {
            ctx.do_next(PullImage)?;
        } else {
            log::info!("The image exists: {}", self.container_info.image_name);
        }
        Ok(())
    }
}

struct PullImage;

#[async_trait]
impl Do<PullImage> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        _: PullImage,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        log::info!("Pulling the image: {}", self.container_info.image_name);
        let opts = Some(CreateImageOptions {
            from_image: self.container_info.image_name.clone(),
            ..Default::default()
        });
        let stream = self
            .docker
            .create_image(opts, None, None)
            .map(PullProgress::from);
        let receiver = Receiver::connect(stream, ctx.recipient());
        self.state = ContainerState::PullingImage(receiver);
        Ok(())
    }
}

#[derive(Debug, Error)]
enum PullError {
    #[error("Docker error: {0}")]
    Bollard(#[from] BollardError),
    #[error("Progress is empty")]
    ProgressEmpty,
    #[error("Current is empty")]
    CurrentEmpty,
    #[error("Total is empty")]
    TotalEmpty,
    #[error("Status is empty")]
    StatusEmpty,
}

#[derive(Debug, From)]
struct PullProgress {
    result: Result<CreateImageInfo, BollardError>,
}

#[async_trait]
impl Do<PullProgress> for ContainerTask {
    type Error = PullError;

    async fn handle(
        &mut self,
        msg: PullProgress,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let info = msg.result?;
        log::info!("Pull: {:?}", info);
        let details = info.progress_detail.ok_or(PullError::ProgressEmpty)?;
        let current = details.current.ok_or(PullError::CurrentEmpty)? * 100;
        let total = details.total.ok_or(PullError::TotalEmpty)?;
        let pct = current / total;
        let stage = info.status.ok_or(PullError::StatusEmpty)?;
        self.pull_progress = pct as u8;
        // TODO: Detect pulling is done
        // TODO: Report about the progress to the bus
        Ok(())
    }

    async fn fallback(
        &mut self,
        err: PullError,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        // TODO: Handle pull errors
        // Restart pulling, etc...
        Ok(())
    }
}

#[derive(Debug, Clone, From)]
struct CreateContainer;

#[async_trait]
impl Do<CreateContainer> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        msg: CreateContainer,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let mut args = Args::default();
        self.mc.args(&mut args);
        let mut ports = Ports::default();
        self.mc.ports(&mut ports);
        let mut envs = Envs::default();
        self.mc.envs(&mut envs);
        /*
        let opts = CreateContainerOptions {
            name: self.inner.container_name.clone(),
            platform: None,
        };

        let mut networks = Networks::default();
        self.inner.image.networks(&mut networks);
        let networks = self.networks_map(networks)?;

        let mut volumes = Volumes::default();
        self.inner.image.volumes(&mut volumes);
        let volumes = volumes_map(volumes.build());

        let mut mounts = Mounts::default();
        self.inner.image.mounts(&mut mounts);
        let mounts = self.mounts_map(mounts.build())?;
        let ports = ports.build();
        let config = Config {
            image: Some(self.inner.image_name.clone()),
            attach_stdin: Some(false),
            attach_stdout: Some(false),
            attach_stderr: Some(false),
            exposed_ports: Some(exposed_ports(&ports)),
            open_stdin: Some(true),
            stdin_once: Some(false),
            tty: Some(true),
            env: Some(envs.build()),
            volumes: Some(volumes),
            cmd: Some(args.build()),
            host_config: Some(HostConfig {
                binds: Some(vec![]),
                network_mode: Some("bridge".to_string()),
                port_bindings: Some(ports_map(&ports)),
                mounts: Some(mounts),
                ..Default::default()
            }),
            networking_config: Some(networks),
            ..Default::default()
        };
        self.docker.create_container(Some(opts), config).await?;
        */
        Ok(())
    }
}

#[derive(Debug, Clone, From)]
struct StartContainer;

#[async_trait]
impl Do<StartContainer> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        msg: StartContainer,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let name = &self.container_info.container_name;
        self.docker.start_container::<String>(name, None).await?;
        Ok(())
    }

    async fn fallback(&mut self, err: Error, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let name = &self.container_info.container_name;
        log::error!("Can't start the container {name}: err");
        let duration = Duration::from_secs(5);
        let notifier = ctx.notifier(StartContainer);
        let timeout = Timeout::spawn(duration, notifier);
        self.state = ContainerState::WaitContainerStart(timeout);
        Ok(())
    }
}

#[derive(Debug, From)]
struct KillContainer;

#[async_trait]
impl Do<KillContainer> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        msg: KillContainer,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let name = &self.container_info.container_name;
        self.docker.kill_container::<String>(name, None).await?;
        Ok(())
    }

    async fn fallback(&mut self, err: Error, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let name = &self.container_info.container_name;
        log::error!("Can't stop the container {name}: err");
        // TODO: Recover after the error
        Ok(())
    }
}

#[derive(Debug, From)]
struct RemoveContainer;

#[async_trait]
impl Do<RemoveContainer> for ContainerTask {
    type Error = Error;

    async fn handle(
        &mut self,
        msg: RemoveContainer,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Self::Error> {
        let opts = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };
        let name = &self.container_info.container_name;
        self.docker.remove_container(name, Some(opts)).await?;
        Ok(())
    }

    async fn fallback(&mut self, err: Error, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let name = &self.container_info.container_name;
        log::error!("Can't remove the container {name}: err");
        // TODO: Recover after the error
        Ok(())
    }
}
