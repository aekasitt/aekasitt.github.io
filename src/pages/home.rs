/* ~~/src/pages/home.rs */

// third-party crates
use leptos::html::Div;
use leptos::prelude::*;
use leptos_router::components::A;

// local modules
use crate::components::hooks::use_horizontal_scroll::{
  HorizontalScrollState, use_horizontal_scroll,
};
use crate::components::statistics::Statistics;
use crate::files::posts::fetch_manifest;
use crate::icons::{ChevronLeft, ChevronRight};

#[component]
pub fn Home() -> impl IntoView {
  let scroll_container_ref = NodeRef::<Div>::new();
  let scroll_ctx = use_horizontal_scroll(scroll_container_ref, None, None);
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
              <div
                class="
                  flex
                  justify-between
                  items-center
                  mb-4
                ">
                <h3
                  class="
                    font-semibold
                    text-lg
                  ">
                  Latest
                </h3>
                <div class="flex gap-2">
                  <button
                    class="
                      bg-secondary
                      border
                      disabled:cursor-not-allowed
                      disabled:opacity-50
                      flex
                      items-center
                      justify-center
                      rounded-full
                      size-8
                    "
                    disabled=move || scroll_ctx.scroll_state.get() == HorizontalScrollState::Start
                    on:click=move |_| scroll_ctx.scroll_by.run(-1)
                    >
                    <ChevronLeft class="size-4" />
                  </button>
                  <button
                    class="
                      bg-secondary
                      border
                      disabled:cursor-not-allowed
                      disabled:opacity-50
                      flex
                      items-center
                      justify-center
                      rounded-full
                      size-8
                    "
                    disabled=move || scroll_ctx.scroll_state.get() == HorizontalScrollState::End
                    on:click=move |_| scroll_ctx.scroll_by.run(1)
                    >
                    <ChevronRight class="size-4" />
                  </button>
                </div>
              </div>
              <div
                class="
                  flex
                  gap-4
                  overflow-x-scroll
                  scroll-smooth
                  snap-mandatory
                  snap-x
                  [scrollbar-width:none]
                  [&::-webkit-scrollbar]:hidden
                "
                node_ref=scroll_container_ref
                on:scroll=move |e| scroll_ctx.on_scroll.run(e)
                >
                <For each=move || entries.get_value() key=|post| post.slug.clone() let:post>
                  {if post.banner.is_some() {
                    view! {
                      <A href=format!("/post/{}/", post.slug)>
                        <div
                          class="
                            flex
                            h-64
                            items-center
                            justify-center
                            relative
                            shrink-0
                            snap-start
                            w-96
                          "
                          >
                          <img
                            alt={post.slug.clone()}
                            class="
                              absolute
                              brightness-75
                              h-full
                              inset-0
                              object-cover
                              rounded-lg
                              w-full
                              -z-20
                            "
                            src={post.banner.clone()}
                            />
                          <span
                            class="
                              font-bold
                              text-2xl
                              text-center
                              text-gray-300
                            ">
                            {post.title.clone()}
                          </span>
                        </div>
                      </A>
                    }.into_any()
                  } else {
                    view! {
                      <A href=format!("/post/{}/", post.slug)>
                        <div
                          class="
                            bg-gray-300
                            flex
                            h-64
                            items-center
                            justify-center
                            rounded-lg
                            shrink-0
                            snap-start
                            w-96
                          "
                          >
                          <span
                            class="
                              font-bold
                              text-2xl
                              text-center
                              text-gray-600
                            ">
                            {post.title.clone()}
                          </span>
                        </div>
                      </A>
                    }.into_any()
                  }}
                </For>
              </div>
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
