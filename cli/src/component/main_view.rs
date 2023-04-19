use crate::component::{Component, ComponentContext};
use crossterm::event::KeyCode;

pub struct MainView {}

impl Component for MainView {
    fn update(&mut self, key: KeyCode) {}

    fn render<'a>(&self, ctx: &mut ComponentContext<'a>) {}
}
