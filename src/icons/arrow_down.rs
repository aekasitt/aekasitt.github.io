/* ~~/src/icons/arrow_down.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn ArrowDown(#[prop(optional, into)] class: String) -> impl IntoView {
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
          M12 6
          V18
          M12 18
          L7 13
          M12 18
          L17 13
        "
        stroke="#000000"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
  }
}
