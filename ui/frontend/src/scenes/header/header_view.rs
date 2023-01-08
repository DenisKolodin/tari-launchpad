use yew::{html, Html};

use super::{ControlButtons, TopMenu};
use crate::{
    scenes::icons,
    widget::{Context, IgnoreAll, Pod, Widget},
};

pub struct HeaderView {}

impl Widget for HeaderView {
    type Msg = IgnoreAll;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div data-tauri-drag-region="" class="header_view">
                <Pod<ControlButtons> />
                <div class="logo">{ icons::logo() }</div>
                <div class="separator" />
                <Pod<TopMenu> />
            </div>
        })
    }
}
