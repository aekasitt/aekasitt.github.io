/* ~~/src/components/ui/textarea.rs */

// third-party crates
use leptos::html;
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Textarea(
  // Styling
  #[prop(into, optional)] class: String,

  // Common HTML attributes
  #[prop(into, optional)] placeholder: Option<String>,
  #[prop(into, optional)] name: Option<String>,
  #[prop(into, optional)] id: Option<String>,
  #[prop(optional)] disabled: bool,
  #[prop(optional)] readonly: bool,
  #[prop(optional)] required: bool,
  #[prop(optional)] autofocus: bool,
  #[prop(into, optional)] rows: Option<u32>,

  // Two-way binding (like bind:value)
  #[prop(into, optional)] bind_value: Option<RwSignal<String>>,

  // Ref for direct DOM access
  #[prop(optional)] node_ref: NodeRef<html::Textarea>,
) -> impl IntoView {
  let merged_class = tw_merge!(
    "
      aria-invalid:border-destructive
      aria-invalid:ring-destructive/20
      bg-transparent
      border
      border-input
      dark:aria-invalid:ring-destructive/40
      dark:bg-input/30
      disabled:cursor-not-allowed
      disabled:opacity-50
      field-sizing-content
      flex
      focus-visible:border-ring
      focus-visible:ring-2
      focus-visible:ring-ring/50
      md:text-sm
      min-h-16
      placeholder:text-muted-foreground
      px-3
      py-2
      rounded-md
      outline-none
      shadow-xs
      text-base
      transition-[color,box-shadow]
      w-full
    ",
    class
  );

  match bind_value {
    Some(signal) => view! {
      <textarea
        autofocus=autofocus
        bind:value=signal
        class=merged_class
        data-name="Textarea"
        disabled=disabled
        id=id
        name=name
        node_ref=node_ref
        placeholder=placeholder
        readonly=readonly
        required=required
        rows=rows
      />
    }
    .into_any(),
    None => view! {
      <textarea
        autofocus=autofocus
        class=merged_class
        data-name="Textarea"
        disabled=disabled
        id=id
        name=name
        node_ref=node_ref
        placeholder=placeholder
        readonly=readonly
        required=required
        rows=rows
      />
    }
    .into_any(),
  }
}
