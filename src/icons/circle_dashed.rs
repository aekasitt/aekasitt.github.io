/* ~~/src/icons/circle_dashed.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn CircleDashed(#[prop(optional, into)] class: String) -> impl IntoView {
  template! {
    <svg
      class=class
      fill="none"
      height="800px"
      viewBox="0 0 24 24"
      width="800px"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      >
      <path
        d="
          M21 12
          C21 16.9706 16.9706 21 12 21
          C7.02944 21 3 16.9706 3 12
          C3 7.02944 7.02944 3 12 3
          C16.9706 3 21 7.02944 21 12Z
        "
        stroke="#000000"
        stroke-dasharray="4 4"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
  }
}
