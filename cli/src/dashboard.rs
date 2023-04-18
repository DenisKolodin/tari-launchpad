use crate::events::{EventHandle, TermEvent};
use anyhow::Error;
use async_trait::async_trait;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Stdout;
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};
use tact::actors::{Actor, ActorContext, Do};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

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
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            terminal: None,
            event_handle: None,
        }
    }
}

#[async_trait]
impl Actor for Dashboard {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let addr = ctx.address().clone();
        let handle = EventHandle::new(addr);
        self.event_handle = Some(handle);
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
            let mut view = View { f };
            view.render();
        })?;
        Ok(())
    }
}

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum Tab {
    Containers,
    Wallet,
}

struct View<'a, 'b> {
    f: &'a mut Frame<'b, CrosstermBackend<Stdout>>,
}

impl<'a, 'b> View<'a, 'b> {
    fn render(&mut self) {
        let _rect = self.render_tabs();
    }

    fn render_tabs(&mut self) -> Rect {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(self.f.size());

        let titles = Tab::iter()
            .map(|s| Spans::from(vec![Span::raw(s.to_string())]))
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            //.select(self.dashboard_state.selected_tab as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        self.f.render_widget(tabs, main_chunks[0]);
        main_chunks[1]
    }
}
