/* ~~/src/pages/home.rs */

// third-party crates
use leptos::prelude::*;

// local modules
use crate::components::latestmill::LatestMill;
use crate::components::statistics::Statistics;
use crate::files::posts::fetch_manifest;

#[component]
pub fn Home() -> impl IntoView {
  let manifest = LocalResource::new(|| async move { fetch_manifest().await });
  view! {
    <Suspense fallback=|| view! { <p>"Loading posts…"</p> }>
      {move || match manifest.get() {
        None => view! { <p>"Loading posts…"</p> }.into_any(),
        Some(Ok(summary)) => {
          let entries = StoredValue::new(summary.entries);
          let updated = StoredValue::new(summary.updated);
          view! {
            <div
              class="
                isolate
                lg:px-16
                pt-8
                px-8
                relative
              ">
              <Statistics
                count=entries.get_value().len()
                updated=updated.get_value()
                />
            </div>
            <div
              class="
                isolate
                lg:px-16
                pt-14
                px-8
                relative
              ">
              <LatestMill entries=entries.get_value()/>
            </div>
          }.into_any()
        },
        Some(Err(err)) => view! {
          <p>"Could not load posts: " {err}</p>
        }.into_any(),
      }}
    </Suspense>
  }
}
