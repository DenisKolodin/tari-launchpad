mod elements;
mod expert;
mod header;
mod main_view;
mod normal;
mod scene;
mod settings;
mod tabs;

use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent};
use derive_more::{From, Into};
pub use main_view::MainView;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

pub trait Component<B: Backend> {
    type State;

    /// A context reference a mutable to modify the frame.
    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pass {
    Up,
    Down,
    Left,
    Right,
    Out,
    None,
}

impl Pass {
    fn any(&self, arr: &[Pass]) -> bool {
        for item in arr {
            if item == self {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, From, Into)]
pub struct ComponentEvent(KeyEvent);

impl ComponentEvent {
    pub fn pass(&self) -> Pass {
        match self.0.code {
            KeyCode::Up | KeyCode::Char('k') => Pass::Up,
            KeyCode::Down | KeyCode::Char('j') => Pass::Down,
            KeyCode::Left | KeyCode::Char('h') => Pass::Left,
            KeyCode::Right | KeyCode::Char('l') => Pass::Right,
            KeyCode::Esc => Pass::Out,
            _ => Pass::None,
        }
    }
}

pub trait Input {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState);
}
