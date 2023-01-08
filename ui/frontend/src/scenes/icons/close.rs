use yew::{html, Html};

/*
pub fn cross(width: u16, height: u16) -> Html {
    html! {
      <svg
        width={width.to_string()}
        height={height.to_string()}
        viewBox="0 0 6 6"
        fill="none"
        data-testid="svg-closecross"
      >
        <path
          d="M4.76796 1.23242L1.23242 4.76796M4.76796 4.76796L1.23242 1.23242"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
        />
      </svg>
    }
}
*/

pub fn close() -> Html {
    // TODO: Move to a separate mod
    html! {
      <svg
        width="8"
        height="8"
        viewBox="0 0 6 6"
        fill="none"
      >
        <path
          d="M4.76796 1.23242L1.23242 4.76796M4.76796 4.76796L1.23242 1.23242"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
        />
      </svg>
    }
}
