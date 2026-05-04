/* ~~/src/components/ui/input.rs */

// third-party crates
use leptos::html;
use leptos::prelude::*;
use strum::AsRefStr;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum InputType {
  #[default]
  Text,
  Email,
  Password,
  Number,
  Tel,
  Url,
  Search,
  Date,
  Time,
  #[strum(serialize = "datetime-local")]
  DatetimeLocal,
  Month,
  Week,
  Color,
  File,
  Hidden,
}

#[component]
pub fn Input(
  // Styling
  #[prop(into, optional)] class: String,

  // Common HTML attributes
  #[prop(default = InputType::default())] r#type: InputType,
  #[prop(into, optional)] placeholder: Option<String>,
  #[prop(into, optional)] name: Option<String>,
  #[prop(into, optional)] id: Option<String>,
  #[prop(into, optional)] title: Option<String>,
  #[prop(into, optional)] autocomplete: Option<String>,
  #[prop(optional)] disabled: bool,
  #[prop(optional)] readonly: bool,
  #[prop(optional)] required: bool,
  #[prop(optional)] autofocus: bool,
  #[prop(optional)] minlength: Option<u16>,

  // Number input attributes
  #[prop(into, optional)] min: Option<String>,
  #[prop(into, optional)] max: Option<String>,
  #[prop(into, optional)] step: Option<String>,

  // Two-way binding (like bind:value)
  #[prop(into, optional)] bind_value: Option<RwSignal<String>>,

  // Ref for direct DOM access
  #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
  let merged_class = tw_merge!(
    "
      bg-transparent
      border
      border-input
      dark:bg-input/30
      disabled:cursor-not-allowed
      disabled:opacity-50
      disabled:pointer-events-none
      file:bg-transparent
      file:border-0
      file:font-medium
      file:inline-flex
      file:h-7
      file:text-foreground
      file:text-sm
      flex
      h-9
      md:text-sm
      min-w-0
      outline-none
      placeholder:text-muted-foreground
      px-3
      py-1
      rounded-md
      selection:bg-primary
      selection:text-primary-foreground
      shadow-xs
      text-base
      text-foreground
      transition-[color,box-shadow]
      w-full
    ",
    "
      focus-visible:border-ring
      focus-visible:ring-ring/50
    ",
    "focus-visible:ring-2",
    "
      aria-invalid:border-destructive
      aria-invalid:ring-destructive/20
      dark:aria-invalid:ring-destructive/40
    ",
    "read-only:bg-muted",
    class
  );

  let type_str = r#type.as_ref();

  match bind_value {
    Some(signal) => view! {
      <input
        autocomplete=autocomplete
        autofocus=autofocus
        bind:value=signal
        class=merged_class
        data-name="Input"
        disabled=disabled
        id=id
        max=max
        min=min
        minlength=minlength
        name=name
        node_ref=node_ref
        placeholder=placeholder
        readonly=readonly
        required=required
        step=step
        title=title
        type=type_str
      />
    }
    .into_any(),
    None => view! {
      <input
        autocomplete=autocomplete
        autofocus=autofocus
        class=merged_class
        data-name="Input"
        disabled=disabled
        id=id
        max=max
        min=min
        minlength=minlength
        name=name
        node_ref=node_ref
        placeholder=placeholder
        readonly=readonly
        required=required
        step=step
        title=title
        type=type_str
      />
    }
    .into_any(),
  }
}
