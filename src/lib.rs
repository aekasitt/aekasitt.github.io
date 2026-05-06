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
    <Meta name="color-scheme" content="dark light"/>
    <Router>
      <Navigation />
      <main>
        <FlatRoutes fallback>
          <Route path=path!("/") view=pages::Home/>
          <Route path=path!("/about") view=move || view! { <Redirect path="/"/> }/>
          <Route path=path!("/post/:slug/") view=pages::Post/>
        </FlatRoutes>
      </main>
    </Router>
  }
}
