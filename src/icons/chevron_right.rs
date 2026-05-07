/* ~~/src/icons/chevron_right.rs */

// third-party crates
use leptos::prelude::*;

#[component]
pub fn ChevronRight(#[prop(optional, into)] class: String) -> impl IntoView {
  view! {
    <svg
      class=class
      version="1.1"
      viewBox="-5.5 0 26 26"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:sketch="http://www.bohemiancoding.com/sketch/ns"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      >
      <g
        fill="currentColor"
        fill-rule="evenodd"
        stroke="none"
        stroke-width="1"
        transform="translate(-474.000000, -1196.000000)"
        >
          <path
            d="
              M488.404,1207.36 
              L477.637,1197.6 
              C476.806,1196.76 475.459,1196.76 474.629,1197.6 
              C473.798,1198.43 473.798,1199.77 474.629,1200.6 
              L483.885,1209 
              L474.629,1217.4 
              C473.798,1218.23 473.798,1219.57 474.629,1220.4 
              C475.459,1221.24 476.806,1221.24 477.637,1220.4 
              L488.404,1210.64 
              C488.854,1210.19 489.052,1209.59 489.015,1209 
              C489.052,1208.41 488.854,1207.81 488.404,1207.36
            "
          />
      </g>
    </svg>
  }
}
