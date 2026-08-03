/* ~~/src/icons/arrow_right.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn ArrowRight(#[prop(optional, into)] class: String) -> impl IntoView {
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
          M6 12
          H18
          M18 12
          L13 7
          M18 12
          L13 17
        "
        stroke="#000000"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
  }
}
