/* ~~/src/pages/post.rs */

// third-party crates
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use pulldown_cmark::Parser;
use pulldown_cmark::html::push_html;

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
          let parser = Parser::new(&post.content);
          let mut output = String::new();
          push_html(&mut output, parser);
          let content = StoredValue::new(output);
          let title = StoredValue::new(post.title);
          view! {
            <article
              class="
                isolate
                lg:px-16
                pt-8
                px-8
                relative
                w-full
              ">
              <Card>
                <CardHeader>
                  <CardTitle
                    class="
                      font-bold
                      text-2xl
                    ">
                    {title.get_value()}
                  </CardTitle>
                  <CardDescription>
                    Sitt Guruvanich
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div
                    class="
                      dark:prose-invert
                      prose
                    "
                    inner_html=content.get_value()
                  />
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
