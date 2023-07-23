mod docker;
mod events;
mod update;

use anyhow::Error;
use crate::container::{ContainerTask, Status, ProcessChanges};
use crate::types::TaskStatus;
use tact::ActorContext;

pub struct ContainerTaskFsm<'a> {
    task: &'a mut ContainerTask,
    ctx: &'a mut ActorContext<ContainerTask>,
}

impl<'a> ContainerTaskFsm<'a> {
    pub fn new(task: &'a mut ContainerTask, ctx: &'a mut ActorContext<ContainerTask>) -> Self {
        Self { task, ctx }
    }

    // TODO: it shouldn't be mutable
    fn get_status(&mut self) -> &Status {
        &self.task.status
    }

    fn set_status(&mut self, status: Status) -> Result<(), Error> {
        self.task.status = status;
        self.ctx.do_next(ProcessChanges)?;
        Ok(())
    }

    fn update_task_status(&mut self, task_status: TaskStatus) -> Result<(), Error> {
        self.task.task_status = task_status;
        log::error!("TODO: Forward the task status to the bus");
        Ok(())
    }
}

