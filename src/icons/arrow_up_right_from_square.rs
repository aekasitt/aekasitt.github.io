/* ~~/src/icons/arrow_up_right_from_square.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn ArrowUpRightFromSquare(#[prop(optional, into)] class: String) -> impl IntoView {
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
          L12 12
          M20 4
          V8.5
          M20 4
          H15.5
          M19 12.5
          V16.8
          C19 17.9201 19 18.4802 18.782 18.908
          C18.5903 19.2843 18.2843 19.5903 17.908 19.782
          C17.4802 20 16.9201 20 15.8 20
          H7.2
          C6.0799 20 5.51984 20 5.09202 19.782
          C4.71569 19.5903 4.40973 19.2843 4.21799 18.908
          C4 18.4802 4 17.9201 4 16.8
          V8.2
          C4 7.0799 4 6.51984 4.21799 6.09202
          C4.40973 5.71569 4.71569 5.40973 5.09202 5.21799
          C5.51984 5 6.07989 5 7.2 5
          H11.5
        "
        stroke="#000000"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
  }
}
