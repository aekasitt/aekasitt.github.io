/* ~~/src/pages.rs */

// third-party crates
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

// local modules
use crate::files::get_post;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
  slug: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
  let query = use_params::<PostParams>();
  let post = move || {
    query
      .get()
      .ok()
      .and_then(|q| q.slug)
      .and_then(|slug| get_post(&slug))
  };
  view! {
    <em>"The world's best content."</em>
    {move || {
      post()
        .map(|post| {
          view! {
            <article>
              <h1>
                {post.title.clone()}
              </h1>
              <p>
                {post.content.clone()}
              </p>
              <Title text=post.title/>
              <Meta name="description" content=post.content/>
            </article>
          }.into_any()
        })
        .unwrap_or_else(|| {
          view! {
            <p>"Post not found."</p>
          }.into_any()
        })
    }}
  }
}
