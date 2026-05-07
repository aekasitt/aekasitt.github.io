/* ~~/src/icons/search.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn Search() -> impl IntoView {
  view! {
    <svg
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      >
      <g
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        >
        <path
          d="
            M11 6
            C13.7614 6 16 8.23858 16 11
            M16.6588 16.6549
            L21 21
            M19 11
            C19 15.4183 15.4183 19 11 19
            C6.58172 19 3 15.4183 3 11
            C3 6.58172 6.58172 3 11 3
            C15.4183 3 19 6.58172 19 11Z
          "
        />
      </g>
    </svg>
  }
}
