use yew::{html, Html};

pub fn minimize() -> Html {
    html! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="8"
          height="2"
          viewBox="0 0 8 2"
          fill="none"
        >
          <path
            d="M1 1H9"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
          />
        </svg>
    }
}
