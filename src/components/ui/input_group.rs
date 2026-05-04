/* ~~/src/components/ui/input_group.rs */

// third-party crates
use leptos::prelude::*;
use leptos_ui::{clx, variants};
use tw_merge::{TwClass, TwVariant, tw_merge};

// local crates
use crate::components::ui::input::{Input, InputType};
use crate::components::ui::textarea::Textarea;

mod components {
  use super::*;
  clx! {
    InputGroupText,
    span,
    "
      flex
      gap-2
      items-center
      text-muted-foreground
      text-sm
      [&_svg]:pointer-events-none
      [&_svg:not([class*='size-'])]:size-4
    "
  }
}

#[component]
pub fn InputGroup(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
  let merged_class = tw_merge!(
    "
      border
      border-input
      dark:bg-input/30
      dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40
      flex
      group/input-group
      h-9
      has-[>textarea]:h-auto
      has-[>[data-align=inline-start]]:[&>input]:pl-2
      has-[>[data-align=inline-end]]:[&>input]:pr-2
      has-[>[data-align=block-start]]:h-auto
      has-[>[data-align=block-start]]:flex-col
      has-[>[data-align=block-start]]:[&>input]:pb-3
      has-[>[data-align=block-end]]:h-auto
      has-[>[data-align=block-end]]:flex-col
      has-[>[data-align=block-end]]:[&>input]:pt-3
      has-[[data-slot=input-group-control]:focus-visible]:border-ring
      has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50
      has-[[data-slot=input-group-control]:focus-visible]:ring-[3px]
      has-[[data-slot][aria-invalid=true]]:ring-destructive/20
      has-[[data-slot][aria-invalid=true]]:border-destructive
      items-center
      min-w-0
      outline-none
      relative
      w-full
      rounded-md
      shadow-xs
      transition-[color,box-shadow]
    ",
    class
  );
  view! {
    <div data-name="InputGroup" data-slot="input-group" role="group" class=merged_class>
      {children()}
    </div>
  }
}

#[component]
pub fn InputGroupAddon(
  #[prop(default = InputGroupAddonAlign::default())] align: InputGroupAddonAlign,
  #[prop(into, optional)] class: String,
  children: Children,
) -> impl IntoView {
  let addon = InputGroupAddonClass { align };
  let align_attr = match align {
    InputGroupAddonAlign::InlineStart => "inline-start",
    InputGroupAddonAlign::InlineEnd => "inline-end",
    InputGroupAddonAlign::BlockStart => "block-start",
    InputGroupAddonAlign::BlockEnd => "block-end",
  };
  let merged_class = addon.with_class(class);
  view! {
    <div
      class=merged_class
      data-align=align_attr
      data-name="InputGroupAddon"
      data-slot="input-group-addon"
      role="group"
      >
      {children()}
    </div>
  }
}

#[derive(TwClass, Default)]
#[tw(class = "
    cursor-text
    flex
    font-medium
    gap-2
    group-data-[disabled=true]/input-group:opacity-50
    h-auto
    items-center
    justify-center
    py-1.5
    select-none [&>svg:not([class*='size-'])]:size-4
    [&>kbd]:rounded-[calc(var(--radius)-5px)]
    text-muted-foreground
    text-sm
  ")]
struct InputGroupAddonClass {
  align: InputGroupAddonAlign,
}

#[derive(TwVariant)]
pub enum InputGroupAddonAlign {
  #[tw(
    default,
    class = "
      has-[>button]:ml-[-0.45rem]
      has-[>kbd]:ml-[-0.35rem]
      order-first
      pl-3
    "
  )]
  InlineStart,
  #[tw(class = "
      has-[>button]:mr-[-0.45rem]
      has-[>kbd]:mr-[-0.35rem]
      order-last
      pr-3
    ")]
  InlineEnd,
  #[tw(class = "
      group-has-[>input]/input-group:pt-2.5
      justify-start
      order-first
      pt-3
      px-3
      w-full
      [.border-b]:pb-3
    ")]
  BlockStart,
  #[tw(
    class = "order-last w-full justify-start px-3 pb-3 [.border-t]:pt-3 group-has-[>input]/input-group:pb-2.5"
  )]
  BlockEnd,
}

variants! {
    InputGroupButton {
        base: "text-sm shadow-none flex gap-2 items-center",
        variants: {
            variant: {
                Ghost: "",
            },
            size: {
                Xs: "h-6 gap-1 px-2 rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-3.5 has-[>svg]:px-2",
                Sm: "h-8 px-2.5 gap-1.5 rounded-md has-[>svg]:px-2.5",
                IconXs: "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0",
                IconSm: "size-8 p-0 has-[>svg]:p-0",
            }
        },
        component: {
            element: button
        }
    }
}

#[component]
pub fn InputGroupInput(
  #[prop(into, optional)] class: String,
  #[prop(default = InputType::default())] r#type: InputType,
  #[prop(into, optional)] placeholder: String,
  #[prop(into, optional)] name: String,
  #[prop(into, optional)] id: String,
  #[prop(into, optional)] title: String,
  #[prop(optional)] disabled: bool,
  #[prop(optional)] readonly: bool,
  #[prop(optional)] required: bool,
  #[prop(optional)] autofocus: bool,
  #[prop(into, optional)] min: String,
  #[prop(into, optional)] max: String,
  #[prop(into, optional)] step: String,
) -> impl IntoView {
  let merged_class = tw_merge!(
    "
      bg-transparent
      border-0
      dark:bg-transparent
      flex-1
      focus-visible:ring-0
      rounded-none
      shadow-none
    ",
    class
  );

  view! {
    <Input
      attr:data-slot="input-group-control"
      autofocus=autofocus
      class=merged_class
      disabled=disabled
      id=id
      max=max
      min=min
      name=name
      placeholder=placeholder
      readonly=readonly
      required=required
      r#type=r#type
      step=step
      title=title
    />
  }
}

#[component]
pub fn InputGroupTextarea(#[prop(into, optional)] class: String) -> impl IntoView {
  let merged_class = tw_merge!(
    "
      bg-transparent
      border-0
      dark:bg-transparent
      flex-1
      focus-visible:ring-0
      py-3
      resize-none
      rounded-none
      shadow-none
    ",
    class
  );

  view! { <Textarea class=merged_class attr:data-slot="input-group-control" /> }
}
