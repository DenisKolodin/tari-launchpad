use anyhow::Error;
use async_trait::async_trait;
use bollard::errors::Error as BollardError;
use bollard::image::CreateImageOptions;
use bollard::models::CreateImageInfo;
use bollard::Docker;
use derive_more::From;
use futures::StreamExt;
use tact::{Actor, ActorContext, Do, Receiver, Recipient};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageInfo {
    pub registry: String,
    pub image_name: String,
    pub tag: String,
}

impl ImageInfo {
    pub fn new(registry: &str, image_name: &str, tag: &str) -> Self {
        Self {
            registry: registry.to_string(),
            image_name: image_name.to_string(),
            tag: tag.to_string(),
        }
    }
}

#[derive(Debug)]
enum ContainerState {
    Idle,
    PullingImage(Receiver),
}

pub struct ContainerTask {
    docker: Docker,
    info: ImageInfo,
    state: ContainerState,
}

impl ContainerTask {
    pub fn new(docker: Docker, info: ImageInfo) -> Self {
        let scope = "tari_scope".to_string();
        let image_name = format!("{}/{}:{}", info.registry, info.image_name, info.tag);
        let container_name = format!("{}_{}", scope, info.image_name);
        Self {
            docker,
            info,
            state: ContainerState::Idle,
        }
    }
}

#[async_trait]
impl Actor for ContainerTask {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        ctx.do_next(CheckImage)?;
        Ok(())
    }
}

struct CheckImage;

#[async_trait]
impl Do<CheckImage> for ContainerTask {
    async fn handle(&mut self, _: CheckImage, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let exist = self
            .docker
            .inspect_image(&self.info.image_name)
            .await
            .is_ok();
        if !exist {
            ctx.do_next(PullImage)?;
        }
        Ok(())
    }
}

struct PullImage;

#[async_trait]
impl Do<PullImage> for ContainerTask {
    async fn handle(&mut self, _: PullImage, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let opts = Some(CreateImageOptions {
            from_image: self.info.image_name.clone(),
            ..Default::default()
        });
        let stream = self
            .docker
            .create_image(opts, None, None)
            .map(PullingProgress::from);
        let receiver = Receiver::connect(stream, ctx.recipient());
        self.state = ContainerState::PullingImage(receiver);
        Ok(())
    }
}

#[derive(From)]
struct PullingProgress {
    result: Result<CreateImageInfo, BollardError>,
}

#[async_trait]
impl Do<PullingProgress> for ContainerTask {
    async fn handle(
        &mut self,
        msg: PullingProgress,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        let info = msg.result?;
        println!("INFO: {:?}", info);
        Ok(())
    }
}
