use derive_more::Deref;
use std::fmt::Debug;

pub trait WorkerStatus: Debug + Default {
    fn is_ready(&self) -> bool;
}

#[derive(Deref)]
pub struct SdmStatus<S> {
    name: String,
    #[deref]
    status: S,
}

impl<S: WorkerStatus> SdmStatus<S> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: S::default(),
        }
    }

    pub fn get(&self) -> &S {
        &self.status
    }

    pub fn set(&mut self, status: S) {
        log::debug!(
            "Set the new status !{}::status={:?}",
            self.name,
            self.status
        );
        self.status = status;
    }

    pub fn update<F>(&mut self, func: F)
    where
        F: FnOnce(&mut S),
    {
        func(&mut self.status);
    }
}
