/* ~~/src/icons/arrow_up.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn ArrowUp(#[prop(optional, into)] class: String) -> impl IntoView {
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
          M12 6
          L7 11
          M12 6
          L17 11
        "
        stroke="#000000"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
  }
}
