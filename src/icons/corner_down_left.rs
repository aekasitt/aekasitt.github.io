/* ~~/src/icons/corner_down_left.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn CornerDownLeft(#[prop(optional, into)] class: String) -> impl IntoView {
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
          M20 4
          V6.4
          C20 9.76032 20 11.4405 19.346 12.7239
          C18.7708 13.8529 17.8529 14.7708 16.7239 15.346
          C15.4405 16 13.7603 16 10.4 16
          H4
          M4 16
          L8 12
          M4 16
          L8 20
        "
        stroke="#000000"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  }
}
