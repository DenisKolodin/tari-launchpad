use anyhow::Error;
use yew::{html, Html};

use crate::{
    scenes::icons,
    states::{local_state::LocalState, remote_state::RemoteState},
    widget::{Context, FromDelta, Widget},
};

pub struct ControlButtons {}

#[derive(Clone)]
pub enum Msg {}

impl FromDelta<LocalState> for Msg {}

impl FromDelta<RemoteState> for Msg {}

impl Widget for ControlButtons {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn on_event(&mut self, _event: Msg, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="control_buttons">
                <button class="close">{ icons::close() }</button>
                <button class="minimize">{ icons::minimize() }</button>
                <button class="maximize">{ icons::maximize() }</button>
            </div>
        })
    }
}
