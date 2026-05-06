/* ~~/src/pages/post.rs */

// third-party crates
use leptos::prelude::*;
use leptos_md::Markdown;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

// local modules
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::files::posts::fetch_post;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
  slug: Option<String>,
}

#[component]
pub fn Post() -> impl IntoView {
  let params = use_params::<PostParams>();
  let resource = LocalResource::new(move || {
    let slug = params.get().ok().and_then(|q| q.slug);
    async move {
      match slug.as_deref() {
        Some(s) => fetch_post(s).await,
        None => Err("missing slug".to_string()),
      }
    }
  });
  view! {
    <Suspense fallback=|| view! { <p>"Loading post…"</p> }>
      {move || match resource.get() {
        None => view! { <p>"Loading post…"</p> }.into_any(),
        Some(Ok(Some(post))) => {
          let title = StoredValue::new(post.title);
          let content = StoredValue::new(post.content);
          view! {
          <article
            class="
              max-w-2xl
              px-auto
            ">
            <Card
              class="
                w-full
              ">
              <CardHeader>
                <CardTitle>
                  {title.get_value()}
                </CardTitle>
                <CardDescription>
                  Sitt Guruvanich
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Markdown content=content.get_value()/>
              </CardContent>
            </Card>
            <Meta name="description" content=content.get_value()/>
            <Title text=title.get_value()/>
          </article>
        }.into_any()
        },
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
