/* ~~/src/lib.rs */

// third-party crates
use leptos::prelude::*;
use leptos_router::components::{FlatRoutes, Redirect, Route, Router};
use leptos_router::path;
#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// local modules
use crate::components::commandbox::CommandBox;
use crate::components::navigation::Navigation;

mod components;
mod files;
mod icons;
mod models;
mod pages;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
  unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
  let (search_toggled, set_search_toggled) = signal(false);
  let command_focused = RwSignal::new(false);
  view! {
    <Router>
      <main class="min-h-screen">
        <Navigation set_search_toggled=set_search_toggled />
        <CommandBox command_focused=command_focused search_toggled=search_toggled />
        <FlatRoutes fallback=|| view! { <p>"Page not found."</p> }.into_view()>
          <Route path=path!("/") view=pages::Home/>
          <Route path=path!("/about") view=move || view! { <Redirect path="/"/> }/>
          <Route path=path!("/post/:slug/") view=pages::Post/>
        </FlatRoutes>
      </main>
    </Router>
  }
}
