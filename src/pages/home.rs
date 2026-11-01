/* ~~/src/pages/home.rs */

// third-party crates
use futures_util::future::try_join;
use leptos::prelude::*;

// local modules
use crate::components::latestmill::LatestMill;
use crate::components::statistics::Statistics;
use crate::files::posts::{fetch_entry_count, fetch_latest_entries};

#[component]
pub fn Home() -> impl IntoView {
    let latest = LocalResource::new(|| async move {
        try_join(fetch_latest_entries(), fetch_entry_count()).await
    });
    view! {
        <Suspense fallback=|| view! { <p>"Loading posts…"</p> }>
            {move || match latest.get() {
                None => view! { <p>"Loading posts…"</p> }.into_any(),
                Some(Ok((content, count))) => {
                    let entries = StoredValue::new(content.entries);
                    let served_by = StoredValue::new(content.served_by);
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
                                count=count
                                served_by=served_by.get_value()
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
