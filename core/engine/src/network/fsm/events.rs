use anyhow::Error;
use bollard::models::TaskStatus;

impl<'a> NetworkTaskFsm<'a> {
    pub fn process_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Created => self.on_created(),
            Event::Destroyed => self.on_destroyed(),
        }
    }

    fn on_created(&mut self) -> Result<(), Error> {
        if let Status::WaitCreating = self.status.get() {
            self.status.set(Status::Active);
            self.update_task_status(TaskStatus::Active)?;
        }
        Ok(())
    }

    fn on_destroyed(&mut self) -> Result<(), Error> {
        if let Status::WaitRemoving = self.status.get() {
            self.status.set(Status::Inactive);
            self.update_task_status(TaskStatus::Inactive)?;
        }
        Ok(())
    }
}
