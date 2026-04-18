/* ~~/src/pages/home.rs */

// third-party crates
use leptos::prelude::*;
use leptos_router::components::A;

// local modules
use crate::components::nav_bar::NavBar;
use crate::files::posts::list_posts;

#[component]
pub fn Home() -> impl IntoView {
  let posts = list_posts();
  view! {
    <NavBar />
    <h1>"My Great Blog"</h1>
    <ul>
      <For each=move || posts.clone() key=|post| post.slug.clone() let:post>
        <li>
          <A href=format!("/post/{}/", post.slug)>{post.title}</A>
        </li>
      </For>
    </ul>
  }
}
