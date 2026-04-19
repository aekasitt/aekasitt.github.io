/* ~~/src/pages/home.rs */

// third-party crates
use leptos::prelude::*;
use leptos_router::components::A;

// local modules
use crate::components::nav_bar::NavBar;
use crate::files::posts::{PostSummary, fetch_post_summaries};

#[component]
pub fn Home() -> impl IntoView {
  let summaries = LocalResource::new(|| async move { fetch_post_summaries().await });

  view! {
    <NavBar />
    <h1>"My Great Blog"</h1>
    <Suspense fallback=|| view! { <p>"Loading posts…"</p> }>
      {move || match summaries.get() {
        None => view! { <p>"Loading posts…"</p> }.into_any(),
        Some(Ok(posts)) => view! {
          <PostList posts/>
        }.into_any(),
        Some(Err(err)) => view! {
          <p>"Could not load posts: " {err}</p>
        }.into_any(),
      }}
    </Suspense>
  }
}

#[component]
fn PostList(posts: Vec<PostSummary>) -> impl IntoView {
  view! {
    <ul>
      <For each=move || posts.clone() key=|post| post.slug.clone() let:post>
        <li>
          <A href=format!("/post/{}/", post.slug)>{post.title.clone()}</A>
        </li>
      </For>
    </ul>
  }
}
