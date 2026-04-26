/* ~~/src/pages/home.rs */

// third-party crates
use icons::{ChevronLeft, ChevronRight};
use leptos::html::Div;
use leptos::prelude::*;
use leptos_router::components::A;

// local modules
use crate::components::hooks::use_horizontal_scroll::{
  HorizontalScrollState, use_horizontal_scroll,
};
use crate::components::statistics::Statistics;
use crate::files::posts::fetch_post_summaries;

#[component]
pub fn Home() -> impl IntoView {
  let scroll_container_ref = NodeRef::<Div>::new();
  let scroll_ctx = use_horizontal_scroll(scroll_container_ref, None, None);
  let summaries = LocalResource::new(|| async move { fetch_post_summaries().await });
  view! {
    <Suspense fallback=|| view! { <p>"Loading posts…"</p> }>
      <div
        class="
          isolate
          lg:px-16
          pt-8
          px-8
          relative
        ">
        <Statistics />
      </div>
      {move || match summaries.get() {
        None => view! { <p>"Loading posts…"</p> }.into_any(),
        Some(Ok(posts)) => view! {
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
              <For each=move || posts.clone() key=|post| post.slug.clone() let:post>
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
                    ">
                    <span
                      class="
                        font-bold
                        text-2xl
                        text-gray-600
                      ">
                      {post.title.clone()}
                    </span>
                  </div>
                </A>
              </For>
            </div>
          </div>
        }.into_any(),
        Some(Err(err)) => view! {
          <p>"Could not load posts: " {err}</p>
        }.into_any(),
      }}
    </Suspense>
  }
}
