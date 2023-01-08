use anyhow::Error;
use yew::Html;

use crate::widget::{AcceptAll, Context, Widget};

pub struct ExpertView {}

impl Widget for ExpertView {
    type Msg = AcceptAll;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn on_event(&mut self, _msg: Self::Msg, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        None
    }
}
