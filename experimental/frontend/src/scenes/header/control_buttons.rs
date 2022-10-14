use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    scenes::{icons, MainView},
    states::{
        local_state::{LocalState, LocalStateDelta, ViewMode, LOCAL_STATE},
        remote_state::RemoteState,
    },
    widget::{Connected, Context, FromDelta, Widget, Pod},
};

pub struct ControlButtons {
    show_icons: bool,
}

#[derive(Clone)]
pub enum Msg {
    ShowIcons(bool),
}

impl FromDelta<LocalState> for Msg {}

impl FromDelta<RemoteState> for Msg {}

impl Widget for ControlButtons {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {
            show_icons: false,
        }
    }

    fn on_event(&mut self, event: Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        match event {
            Msg::ShowIcons(value) => {
                self.show_icons = value;
                ctx.redraw();
            }
        }
        Ok(())
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="control_buttons">
                <button class="close">{ icons::close::cross(12, 12) }</button>
                <button class="close">{ icons::close::cross(12, 12) }</button>
                <button class="close">{ icons::close::cross(12, 12) }</button>
            </div>
        })
    }
}
