/* ~~/src/icons/marker.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn Marker(#[prop(optional, into)] class: String) -> impl IntoView {
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
          M7.00002 7
          L8.73728 5.26274
          C9.52931 4.47071 9.92533 4.07469 10.382 3.92632
          C10.7837 3.7958 11.2164 3.7958 11.6181 3.92632
          C12.0747 4.07469 12.4707 4.47071 13.2628 5.26274
          L14 6.00012
          M10 10
          L14 14
          M9.50005 18.5002
          C10.1581 17.8422 16.644 11.3563 20.004 7.99627
          C21.1086 6.89166 21.1046 5.10475 20.0001 4.0001
          C18.8954 2.89541 17.1084 2.89137 16.0038 3.99603
          C12.6437 7.35611 6.15797 13.8419 5.49993 14.4999
          C3.71534 16.2843 2.64644 20.6464 2.99993 20.9999
          C3.35341 21.3533 7.66067 20.3396 9.50005 18.5002Z
        "
        stroke="#000000"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
      />
    </svg>
  }
}
