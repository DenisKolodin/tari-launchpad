use yew::{html, Html};

use crate::widget::{Context, IgnoreAll, Widget};

pub struct TopMenu {}

impl Widget for TopMenu {
    type Msg = IgnoreAll;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="top_menu">
                <div class="menu_item">{ settings_icon() }{ "Settings" }</div>
                <div class="menu_item">{ "Expert view" }</div>
            </div>
        })
    }
}

fn settings_icon() -> Html {
    html! {
      <svg
        width="1em"
        height="1em"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        data-testid="svg-setting2"
      >
        <path
          d="M13.752 4.693c0-.835-.61-1.544-1.436-1.67a2.111 2.111 0 0 0-.632 0 1.689 1.689 0 0 0-1.436 1.67v1.181c-.471.135-.92.322-1.34.556l-.836-.835a1.689 1.689 0 0 0-2.196-.166c-.17.126-.32.277-.447.447a1.689 1.689 0 0 0 .166 2.196l.835.835a6.33 6.33 0 0 0-.556 1.341h-1.18c-.836 0-1.545.61-1.67 1.436-.032.21-.032.423 0 .632a1.689 1.689 0 0 0 1.67 1.436h1.18c.135.471.322.92.556 1.34l-.835.836c-.59.59-.66 1.523-.166 2.196.126.17.277.32.447.447a1.688 1.688 0 0 0 2.196-.166l.835-.835c.42.234.87.421 1.341.556v1.18c0 .836.61 1.545 1.436 1.67.21.032.423.032.632 0a1.688 1.688 0 0 0 1.436-1.67v-1.18a6.335 6.335 0 0 0 1.34-.556l.836.835c.59.59 1.523.66 2.196.166a2.11 2.11 0 0 0 .447-.447 1.688 1.688 0 0 0-.166-2.196l-.835-.835c.234-.42.421-.87.556-1.341h1.18c.836 0 1.545-.61 1.67-1.436.032-.21.032-.423 0-.632a1.688 1.688 0 0 0-1.67-1.436h-1.18a6.332 6.332 0 0 0-.556-1.34l.835-.836c.59-.59.66-1.524.166-2.196a2.11 2.11 0 0 0-.447-.447 1.689 1.689 0 0 0-2.196.166l-.835.835a6.328 6.328 0 0 0-1.341-.556v-1.18Z"
          stroke="currentColor"
          strokeWidth={1.5}
          strokeLinecap="round"
          strokeLinejoin="round"
        />
        <path
          d="M9.5 12a2.5 2.5 0 1 1 5 0 2.5 2.5 0 0 1-5 0Z"
          stroke="currentColor"
          strokeWidth={1.5}
        />
      </svg>
    }
}
