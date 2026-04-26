/* ~~/src/components/ui/popover.rs */

// third-party crates
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

// local crates
use crate::components::hooks::use_random::use_random_id;

mod components {
  use super::*;
  clx! {
    PopoverTitle,
    h3,
    "
      font-medium
      leading-none
    ",
    "mb-3"
  }
  clx! {
    PopoverDescription,
    p,
    "
      text-muted-foreground
      text-sm
    "
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverAlign {
  Start,
  StartOuter,
  End,
  EndOuter,
  #[default]
  Center,
}

#[derive(Clone)]
struct PopoverContext {
  anchor_name: String,
  target_id: String,
}

#[component]
pub fn Popover(
  children: Children,
  #[prop(optional, into)] anchor_name: Option<String>,
  #[prop(default = PopoverAlign::default())] align: PopoverAlign,
) -> impl IntoView {
  let popover_id = use_random_id();
  let popover_anchor_name = anchor_name.unwrap_or_else(|| format!("--anchor_{}", popover_id));
  let popover_target_id = format!("popover_{}", popover_id);

  let (position_styles, transform_origin) = match align {
    PopoverAlign::Start => (
      "left: anchor(left);
                bottom: anchor(top);
                margin-bottom: 8px;
                @position-try(flip-block) {
                top: anchor(bottom);
                bottom: auto;
                margin-top: 8px;
                margin-bottom: 0;
                }"
      .to_string(),
      "left top".to_string(),
    ),
    PopoverAlign::StartOuter => (
      "right: anchor(left);
                top: anchor(top);
                margin-right: 8px;
                @position-try(flip-block) {
                top: anchor(bottom);
                margin-top: 8px;
                }"
      .to_string(),
      "right top".to_string(),
    ),
    PopoverAlign::End => (
      "right: anchor(right);
                bottom: anchor(top);
                margin-bottom: 8px;
                @position-try(flip-block) {
                top: anchor(bottom);
                bottom: auto;
                margin-top: 8px;
                margin-bottom: 0;
                }"
      .to_string(),
      "right top".to_string(),
    ),
    PopoverAlign::EndOuter => (
      "left: anchor(right);
                top: anchor(top);
                margin-left: 8px;
                @position-try(flip-block) {
                top: anchor(bottom);
                margin-top: 8px;
                }"
      .to_string(),
      "left top".to_string(),
    ),
    PopoverAlign::Center => (
      "position-area: block-start;".to_string(),
      "center top".to_string(),
    ),
  };

  let ctx = PopoverContext {
    anchor_name: popover_anchor_name.clone(),
    target_id: popover_target_id.clone(),
  };
  view! {
    <leptos::context::Provider value=ctx>
      <style>
        {format!(
            "
          #{popover_target_id} {{
          position-anchor: {popover_anchor_name};
          inset: auto;
          {position_styles}
          position-try-fallbacks: flip-block;
          position-try-order: most-height;
          position-visibility: anchors-visible;
          
          /* Open State */
          &:popover-open {{
          opacity: 1;
          transform: scale(1) translateY(0px);
          
          @starting-style {{
          opacity: 0;
          transform: scale(0.95) translateY(-2px);
          }}
          }}
          
          /* Closed State */
          & {{
          transition: 
          display 0.2s allow-discrete,
          overlay 0.2s allow-discrete,
          transform 0.15s cubic-bezier(0.16, 1, 0.3, 1),
          opacity 0.15s ease-out;
          opacity: 0;
          transform: scale(0.95) translateY(-2px);
          transform-origin: var(--popover-transform-origin, {transform_origin});
          }}
          }}
          ",
        )}
      </style>

      <div>{children()}</div>
    </leptos::context::Provider>
  }
}

#[component]
pub fn PopoverTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
  let ctx = expect_context::<PopoverContext>();
  let button_class = tw_merge!(
    "
      bg-background
      border
      border-input
      disabled:cursor-not-allowed
      disabled:opacity-50
      focus-visible:outline-hidden
      focus-visible:ring-1
      focus-visible:ring-ring
      font-medium
      h-9
      hover:bg-accent
      hover:text-accent-foreground
      inline-flex
      items-center
      justify-center
      px-4
      py-2
      rounded-md
      text-sm
      transition-colors
      w-fit
      whitespace-nowrap
      [&_svg:not(:last-child)]:mr-2
      [&_svg:not(:first-child)]:ml-2 
    ",
    class
  );
  view! {
    <button
      class=button_class
      popovertarget=ctx.target_id
      style=format!("anchor-name: {}", ctx.anchor_name)
      tabindex="0"
      type="button"
    >
      {children()}
    </button>
  }
}

#[component]
pub fn PopoverContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
  let ctx = expect_context::<PopoverContext>();
  let class = tw_merge!(
    "
      bg-card
      border
      min-h-[150px]
      my-[1ch]
      overflow-visible
      p-4
      relative
      rounded-md
      shadow-md
      w-[250px]
      z-50
    ",
    class
  );

  let target_id = ctx.target_id.clone();

  view! {
    <div class=class id=ctx.target_id popover="auto">
      {children()}
    </div>
    <script>
      {format!(
        r#"
          (() => {{
            const p = document.getElementById('{target_id}');
            if (!p || p.dataset.init) return;
            p.dataset.init = '1';
            p.addEventListener('click', e => e.target.closest('[data-name="CommandItem"]') && p.hidePopover());
          }})();
        "#,
      )}
    </script>
  }
}
