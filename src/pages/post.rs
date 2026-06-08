/* ~~/src/pages/post.rs */

// third-party crates
use leptos::prelude::*;
use leptos_md::{CodeBlockTheme, Markdown, MarkdownOptions};
use leptos_meta::Title;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use wasm_bindgen::prelude::*;

// local modules
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::files::posts::fetch_post;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
  slug: Option<String>,
}

#[wasm_bindgen(inline_js = r#"
export function highlight() {
  requestAnimationFrame(() => {
    if (window.Prism)
      window.Prism.highlightAll();
  });
}"#)]
extern "C" {
  fn highlight();
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
  Effect::new(move |_| {
    if resource.get().and_then(|r| r.ok()).flatten().is_some() {
      highlight();
    }
  });
  view! {
    <Suspense fallback=|| view! { <p>"Loading post…"</p> }>
      {move || match resource.get() {
        None => view! { <p>"Loading post…"</p> }.into_any(),
        Some(Ok(Some(post))) => {
          let content = StoredValue::new(post.content);
          let options = MarkdownOptions::new()
            .without_code_theme()        // No Tailwind theme (let highlighter handle it)
            .with_language_classes(true); // Emit language-xxx classes (default)
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
                <CardContent
                  class="
                    dark:prose-invert
                    max-w-none
                    prose
                  ">
                  <Markdown
                    content=content.get_value()
                    options=options
                  />
                </CardContent>
              </Card>
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
