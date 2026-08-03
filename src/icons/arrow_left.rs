/* ~~/src/icons/arrow_left.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn ArrowLeft(#[prop(optional, into)] class: String) -> impl IntoView {
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
          M6 12
          L11 7
          M6 12
          L11 17
        "
        stroke="#000000"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  }
}
