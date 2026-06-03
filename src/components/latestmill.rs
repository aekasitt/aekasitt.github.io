/* ~~/src/components/latestmill.rs */

// third-party crates
use leptos::html::Div;
use leptos::prelude::*;
use leptos_router::components::A;

// local modules
use crate::components::hooks::use_horizontal_scroll::{
  HorizontalScrollState, use_horizontal_scroll,
};
use crate::files::posts::Entry;
use crate::icons::{ChevronLeft, ChevronRight};

#[component]
pub fn LatestMill(#[prop(into)] entries: Vec<Entry>) -> impl IntoView {
  let scroll_container_ref = NodeRef::<Div>::new();
  let scroll_ctx = use_horizontal_scroll(scroll_container_ref, None, None);
  view! {
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
      <For each=move || entries.clone() key=|post| post.slug.clone() let:post>
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
                    brightness-60
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
  }
}
