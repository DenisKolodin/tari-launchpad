use anyhow::Error;
use async_trait::async_trait;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Stdout;
use tact::actors::{Actor, ActorContext};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

type Term = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Error)]
pub enum DashboardError {
    #[error("Terminal is not connected")]
    NoTerminal,
}

pub struct Dashboard {
    terminal: Option<Term>,
}

impl Dashboard {
    pub fn new() -> Self {
        Self { terminal: None }
    }
}

#[async_trait]
impl Actor for Dashboard {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    async fn finalize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        disable_raw_mode()?;
        let mut terminal = self
            .terminal
            .take()
            .ok_or_else(|| DashboardError::NoTerminal)?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }
}
