use crate::component::{Component, Input, MainView};
use crate::events::{EventHandle, TermEvent};
use anyhow::Error;
use async_trait::async_trait;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Stdout;
use tact::actors::{Actor, ActorContext, Do};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

type Term = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Error)]
pub enum DashboardError {
    #[error("Terminal is not connected")]
    NoTerminal,
    #[error("Events thread is not started")]
    NoEvents,
}

pub struct Dashboard {
    terminal: Option<Term>,
    event_handle: Option<EventHandle>,
    main_view: MainView,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            terminal: None,
            event_handle: None,
            main_view: MainView::new(),
        }
    }
}

#[async_trait]
impl Actor for Dashboard {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        self.terminal = Some(terminal);
        let addr = ctx.address().clone();
        let handle = EventHandle::new(addr);
        self.event_handle = Some(handle);
        ctx.do_next(Redraw)?;
        Ok(())
    }

    async fn finalize(&mut self, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
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

#[async_trait]
impl Do<TermEvent> for Dashboard {
    async fn handle(
        &mut self,
        event: TermEvent,
        ctx: &mut ActorContext<Self>,
    ) -> Result<(), Error> {
        match event {
            TermEvent::Event(event) => {
                if let Event::Key(key) = event {
                    if let KeyCode::Char('q') = key.code {
                        self.event_handle
                            .as_mut()
                            .ok_or_else(|| DashboardError::NoEvents)?
                            .interrupt();
                    }
                    self.main_view.on_input(key.code);
                }
                ctx.do_next(Redraw)?;
            }
            TermEvent::End => {
                ctx.shutdown();
            }
        }
        Ok(())
    }
}

struct Redraw;

#[async_trait]
impl Do<Redraw> for Dashboard {
    async fn handle(&mut self, _event: Redraw, _ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let terminal = self
            .terminal
            .as_mut()
            .ok_or_else(|| DashboardError::NoTerminal)?;
        terminal.draw(|f| {
            self.main_view.draw(f, f.size());
        })?;
        Ok(())
    }
}
