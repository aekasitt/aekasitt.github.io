/* ~~/src/lib.rs */

// third-party crates
use leptos::prelude::*;
use leptos_meta::{Meta, Title, provide_meta_context};
use leptos_router::components::{FlatRoutes, Redirect, Route, Router};
use leptos_router::path;

// local modules
use crate::components::navigation::Navigation;

mod components;
mod files;
mod icons;
mod models;
mod pages;

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  let fallback = || view! { <p>"Page not found."</p> }.into_view();
  view! {
    <Title text="Guru's Gazette"/>
    <Meta
      content="dark light"
      name="color-scheme"
      />
    <Meta
      content="Collection of Notes written by Sitt Guruvanich"
      name="og:description"
      />
    <Meta
      content="https://aekasitt.github.io/assets/opengraph.png"
      name="og:image"
      />
    <Meta
      content="Guru's Gazette"
      name="og:title"
      />
    <Meta
      content="website"
      name="og:type"
      />
    <Meta
      content="https://aekasitt.github.io"
      name="og:url"
      />
    <Router>
      <main class="min-h-screen">
        <Navigation />
        <FlatRoutes fallback>
          <Route path=path!("/") view=pages::Home/>
          <Route path=path!("/about") view=move || view! { <Redirect path="/"/> }/>
          <Route path=path!("/post/:slug/") view=pages::Post/>
        </FlatRoutes>
      </main>
    </Router>
  }
}
