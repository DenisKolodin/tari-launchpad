use super::{ContainerTask, Status};
use anyhow::Error;

impl ContainerTask {
    pub async fn process_update(&mut self) -> Result<(), Error> {
        match &self.status {
            Status::InitialState => self.do_initial_state().await,
            Status::PullingImage { .. } => self.do_pulling().await,
            Status::CleanDangling => self.do_clean_dangling().await,
            Status::WaitContainerKilled => self.do_wait_container_killed().await,
            Status::WaitContainerRemoved => self.do_wait_container_removed().await,
            Status::Idle => self.do_idle().await,
            Status::CreateContainer => self.do_create_container().await,
            Status::WaitContainerCreated => self.do_wait_container_created().await,
            Status::StartContainer => self.do_start_container().await,
            Status::WaitContainerStarted => self.do_wait_container_started().await,
            Status::Active { .. } => self.do_active().await,
            Status::DropImage => self.do_drop_image().await,
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        /*
        self.update_task_status(TaskStatus::Inactive)?;

        log::debug!("Cheking image {} ...", self.inner.image_name);
        if self.image_exists().await {
            self.clean_dangling()?;
        } else {
            self.start_pulling()?;
        }
        */
        Ok(())
    }

    fn clean_dangling(&mut self) -> Result<(), Error> {
        /*
        log::debug!("Image {} exists. Skip pulling.", self.inner.image_name);
        let progress = TaskProgress::new("Cleaning...");
        self.update_task_status(TaskStatus::Progress(progress))?;
        self.status.set(Status::CleanDangling);
        */
        Ok(())
    }

    fn start_pulling(&mut self) -> Result<(), Error> {
        /*
        log::debug!("Image {} doesn't exist. Pulling.", self.inner.image_name);
        let progress = TaskProgress::new("Pulling...");
        self.update_task_status(TaskStatus::Progress(progress))?;
        let progress = self.pull();
        self.status.set(Status::PullingImage { progress });
        */
        Ok(())
    }

    async fn do_pulling(&mut self) -> Result<(), Error> {
        /*
        if self.image_exists().await {
            // Just loaded, container can't be exist
            self.status.set(Status::Idle);
            self.update_task_status(TaskStatus::Inactive)?;
        }
        */
        Ok(())
    }

    async fn do_clean_dangling(&mut self) -> Result<(), Error> {
        /*
        log::debug!("Cheking container {} ...", self.inner.container_name);
        let state = self.container_state().await;
        match state {
            ContainerState::Running => {
                log::debug!("Container {} is running. Terminating it.", self.inner.container_name);
                self.try_kill_container().await?;
                self.status.set(Status::WaitContainerKilled);
            },
            ContainerState::NotRunning => {
                log::debug!("Container {} is not running. Removing it.", self.inner.container_name);
                self.try_remove_container().await?;
                self.status.set(Status::WaitContainerRemoved);
            },
            ContainerState::NotFound => {
                log::debug!("Container {} doesn't exist.", self.inner.container_name);
                self.status.set(Status::Idle);
                self.update_task_status(TaskStatus::Inactive)?;
            },
        }
        */
        Ok(())
    }

    async fn do_wait_container_killed(&mut self) -> Result<(), Error> {
        // TODO: Wait interval
        Ok(())
    }

    async fn do_wait_container_removed(&mut self) -> Result<(), Error> {
        // TODO: Wait interval
        Ok(())
    }

    async fn do_idle(&mut self) -> Result<(), Error> {
        /*
        if self.force_pull {
            self.force_pull = false;
            self.status.set(Status::DropImage);
            let progress = TaskProgress::new("Removing image...");
            self.update_task_status(TaskStatus::Progress(progress))?;
            Ok(())
        } else if self.should_be_active() {
            self.force_restart = false;
            log::debug!("Preparing a container {} to start...", self.inner.container_name);
            self.status.set(Status::CreateContainer);
            self.update_task_status(TaskStatus::Pending)?;
            Ok(())
        } else {
            Ok(())
        }
        */
        Ok(())
    }

    async fn do_create_container(&mut self) -> Result<(), Error> {
        /*
        log::debug!("Trying to create container {} ...", self.inner.container_name);
        // TODO: Process the result as well
        self.try_create_container().await?;
        self.status.set(Status::WaitContainerCreated);
        */
        Ok(())
    }

    async fn do_wait_container_created(&mut self) -> Result<(), Error> {
        // TODO: Check timeout
        Ok(())
    }

    async fn do_start_container(&mut self) -> Result<(), Error> {
        /*
        if let Err(err) = self.try_start_container().await {
            self.sender().send_error(err.to_string())?;
            self.try_remove_container().await?;
            self.status.set(Status::WaitContainerRemoved);
        } else {
            self.status.set(Status::WaitContainerStarted);
            self.update_task_status(TaskStatus::Pending)?;
        }
        */
        Ok(())
    }

    async fn do_active(&mut self) -> Result<(), Error> {
        /*
        if !self.should_be_active() || self.should_be_restarted() {
            self.status.set(Status::CleanDangling);
        }
        */
        Ok(())
    }

    async fn do_wait_container_started(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn do_drop_image(&mut self) -> Result<(), Error> {
        // self.try_remove_image().await
        Ok(())
    }
}
