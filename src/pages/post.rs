/* ~~/src/pages/post.rs */

// third-party crates
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

// local modules
use crate::files::posts::fetch_post;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
  slug: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
  let params = use_params::<PostParams>();
  let post_res = LocalResource::new(move || {
    let slug = params.get().ok().and_then(|q| q.slug);
    async move {
      match slug.as_deref() {
        Some(s) => fetch_post(s).await,
        None => Err("missing slug".to_string()),
      }
    }
  });

  view! {
    <em>"The world's best content."</em>
    <Suspense fallback=|| view! { <p>"Loading post…"</p> }>
      {move || match post_res.get() {
        None => view! { <p>"Loading post…"</p> }.into_any(),
        Some(Ok(Some(post))) => view! {
          <article>
            <h1>
              {post.title.clone()}
            </h1>
            <p>
              {post.content.clone()}
            </p>
            <Title text=post.title/>
            <Meta name="description" content=post.content.clone()/>
          </article>
        }.into_any(),
        Some(Ok(None)) => view! {
          <p>"Post not found."</p>
        }.into_any(),
        Some(Err(err)) => view! {
          <p>"Could not load post: " {err}</p>
        }.into_any(),
      }}
    </Suspense>
  }
}
