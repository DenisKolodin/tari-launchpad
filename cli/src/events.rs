use crate::dashboard::Dashboard;
use anyhow::Error;
use crossterm::event::{poll, read, Event};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::JoinHandle;
use std::time::Duration;
use tact::Address;

pub enum TermEvent {
    Event(Event),
    End,
}

pub struct EventHandle {
    handle: JoinHandle<Result<(), Error>>,
    interrupted: Arc<AtomicBool>,
}

impl EventHandle {
    pub fn new(addr: Address<Dashboard>) -> Self {
        let interrupted = Arc::new(AtomicBool::new(false));
        let handle = std::thread::spawn({
            let interrupted = interrupted.clone();
            move || -> Result<(), Error> {
                while !interrupted.load(Ordering::Relaxed) {
                    let duration = Duration::from_millis(200);
                    let has_event = poll(duration)?;
                    if has_event {
                        let event = read()?;
                        addr.send(TermEvent::Event(event))?;
                    }
                }
                addr.send(TermEvent::End)?;
                Ok(())
            }
        });
        Self {
            handle,
            interrupted,
        }
    }

    pub fn interrupt(&mut self) {
        self.interrupted.store(true, Ordering::Relaxed);
    }
}
