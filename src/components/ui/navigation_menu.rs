/* ~~/src/components/ui/navigation_menu.rs */

// third-party crates
use icons::ChevronDown;
use leptos::context::Provider;
use leptos::prelude::*;
use tw_merge::*;

// local modules
use crate::components::hooks::use_random::use_random_id_for;

pub fn navigation_menu_trigger_style() -> &'static str {
  "
    bg-background
    data-[state=open]:bg-accent/50
    disabled:opacity-50
    disabled:pointer-events-none
    focus:bg-accent
    focus:outline-none
    focus:text-accent-foreground
    font-medium
    group
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
    w-max
  "
}

#[derive(Clone)]
struct NavigationMenuContext {
  menu_id: String,
}

#[derive(Clone)]
struct NavigationMenuItemContext {
  item_id: String,
}

/* ========================================================== */
/*             NAVIGATION MENU             */
/* ========================================================== */

/// Root navigation wrapper. All content panels are absolutely positioned
/// relative to this element, creating a shared viewport effect without portaling.
#[component]
pub fn NavigationMenu(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
  let menu_id = use_random_id_for("navmenu");
  let ctx = NavigationMenuContext {
    menu_id: menu_id.clone(),
  };
  let class = tw_merge!(
    "relative z-10 flex max-w-max flex-1 items-center justify-center",
    class
  );

  view! {
    <Provider value=ctx>
      // Directional slide animations for content panels
      <style>
        "
        @keyframes navFromStart {
          from { opacity: 0; transform: translateX(-16px); }
          to   { opacity: 1; transform: translateX(0); }
        }
        @keyframes navFromEnd {
          from { opacity: 0; transform: translateX(16px); }
          to   { opacity: 1; transform: translateX(0); }
        }
        @keyframes navToStart {
          from { opacity: 1; transform: translateX(0); }
          to   { opacity: 0; transform: translateX(-16px); }
        }
        @keyframes navToEnd {
          from { opacity: 1; transform: translateX(0); }
          to   { opacity: 0; transform: translateX(16px); }
        }
        @keyframes navFadeIn {
          from { opacity: 0; transform: scale(0.96) translateY(-4px); }
          to   { opacity: 1; transform: scale(1) translateY(0); }
        }
        [data-nav-content][data-motion='from-start'] { animation: navFromStart 200ms ease-out; }
        [data-nav-content][data-motion='from-end']   { animation: navFromEnd 200ms ease-out; }
        [data-nav-content][data-motion='to-start']   { animation: navToStart 200ms ease-out forwards; }
        [data-nav-content][data-motion='to-end']   { animation: navToEnd 200ms ease-out forwards; }
        [data-nav-content][data-state='open']:not([data-motion]) { animation: navFadeIn 200ms ease-out; }
        "
      </style>

      <nav data-name="NavigationMenu" data-nav-menu=menu_id.clone() class=class>
        {children()}
      </nav>

      // Single JS coordinator for all hover interactions and directional animation
      <script>
        {format!(
          r#"
          (function() {{
            const setup = () => {{
              const menuRoot = document.querySelector('[data-nav-menu="{menu_id}"]');
              if (!menuRoot) {{ setTimeout(setup, 50); return; }}
              if (menuRoot.hasAttribute('data-js-initialized')) return;
              menuRoot.setAttribute('data-js-initialized', 'true');

              const triggers = [...menuRoot.querySelectorAll('[data-nav-trigger]')];
              const getContent = (id) => menuRoot.querySelector('[data-nav-content="' + id + '"]');

              let activeItemId = null;
              let activeIndex  = -1;
              let hideTimer;

              const openItem = (trigger, idx) => {{
                clearTimeout(hideTimer);
                const itemId  = trigger.getAttribute('data-nav-trigger');
                const content = getContent(itemId);
                if (!content || activeItemId === itemId) return;

                // Close the previous panel with an exit animation
                if (activeItemId) {{
                  const prevContent = getContent(activeItemId);
                  if (prevContent) {{
                    prevContent.setAttribute('data-motion', idx > activeIndex ? 'to-start' : 'to-end');
                    const prev = prevContent;
                    setTimeout(() => {{
                      prev.setAttribute('data-state', 'closed');
                      prev.removeAttribute('data-motion');
                    }}, 200);
                  }}
                  const prevTrigger = triggers[activeIndex];
                  if (prevTrigger) prevTrigger.setAttribute('data-state', 'closed');
                }}

                // Open the new panel with an entry animation
                if (activeItemId) {{
                  content.setAttribute('data-motion', idx > activeIndex ? 'from-end' : 'from-start');
                }} else {{
                  content.removeAttribute('data-motion');
                }}
                content.setAttribute('data-state', 'open');
                trigger.setAttribute('data-state', 'open');
                activeItemId = itemId;
                activeIndex  = idx;
              }};

              const closeAll = (delay) => {{
                hideTimer = setTimeout(() => {{
                  triggers.forEach(t => t.setAttribute('data-state', 'closed'));
                  if (activeItemId) {{
                    const content = getContent(activeItemId);
                    if (content) {{
                      content.setAttribute('data-state', 'closed');
                      content.removeAttribute('data-motion');
                    }}
                  }}
                  activeItemId = null;
                  activeIndex  = -1;
                }}, delay);
              }};

              triggers.forEach((trigger, idx) => {{
                trigger.addEventListener('mouseenter', () => openItem(trigger, idx));
                trigger.addEventListener('mouseleave', () => closeAll(150));
              }});

              menuRoot.querySelectorAll('[data-nav-content]').forEach(content => {{
                content.addEventListener('mouseenter', () => clearTimeout(hideTimer));
                content.addEventListener('mouseleave', () => closeAll(150));
              }});

              document.addEventListener('click', (e) => {{
                if (!menuRoot.contains(e.target)) closeAll(0);
              }});

              document.addEventListener('keydown', (e) => {{
                if (e.key === 'Escape') closeAll(0);
              }});
            }};

            if (document.readyState === 'loading') {{
              document.addEventListener('DOMContentLoaded', setup);
            }} else {{
              setup();
            }}
          }})();
          "#,
          menu_id = menu_id,
        )}
      </script>
    </Provider>
  }
}

#[component]
pub fn NavigationMenuList(
  children: Children,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let class = tw_merge!(
    "
      flex
      flex-1
      gap-1
      group
      items-center
      justify-center
      list-none
    ",
    class
  );

  view! {
    <ul data-name="NavigationMenuList" class=class>
      {children()}
    </ul>
  }
}

/// NOTE: intentionally has NO `position: relative` so that NavigationMenuContent
/// (with `position: absolute`) escapes to the <nav> root, making all panels
/// appear at the same position → shared viewport effect.
#[component]
pub fn NavigationMenuItem(children: Children) -> impl IntoView {
  let item_id = use_random_id_for("navitem");
  let item_ctx = NavigationMenuItemContext { item_id };

  view! {
    <Provider value=item_ctx>
      <li data-name="NavigationMenuItem">{children()}</li>
    </Provider>
  }
}

#[component]
pub fn NavigationMenuTrigger(
  children: Children,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let item_ctx = expect_context::<NavigationMenuItemContext>();
  let menu_ctx = expect_context::<NavigationMenuContext>();

  let class = tw_merge!(
    "
      bg-background
      cursor-default
      data-[state=open]:bg-accent/50
      disabled:opacity-50
      disabled:pointer-events-none
      focus:bg-accent
      focus:outline-none
      focus:text-accent-foreground
      font-medium
      group
      h-9
      hover:bg-accent
      hover:text-accent-foreground
      inline-flex
      items-center
      justify-center
      px-4
      py-2
      rounded-md
      select-none
      text-sm
      transition-colors
      w-max
    ",
    class
  );

  view! {
    <button
      type="button"
      data-name="NavigationMenuTrigger"
      data-nav-trigger=item_ctx.item_id
      data-nav-menu=menu_ctx.menu_id
      class=class
      data-state="closed"
    >
      {children()}
      <ChevronDown
        class="
          duration-300
          group-data-[state=open]:rotate-180
          ml-1
          relative
          size-3
          top-[1px]
          transition
        "/>
    </button>
  }
}

/// Absolutely positioned relative to NavigationMenu (not NavigationMenuItem),
/// so all content panels share the same anchor point below the menu bar.
#[component]
pub fn NavigationMenuContent(
  children: Children,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let ctx = expect_context::<NavigationMenuItemContext>();

  let class = tw_merge!(
    "
      absolute
      bg-popover
      border
      data-[state=closed]:hidden
      left-0
      md:w-auto
      mt-1.5
      p-4
      rounded-md
      shadow-md
      top-full
      w-full
      z-50
    ",
    class
  );

  view! {
    <div data-name="NavigationMenuContent" data-nav-content=ctx.item_id class=class data-state="closed">
      {children()}
    </div>
  }
}

#[component]
pub fn NavigationMenuLink(
  children: Children,
  #[prop(optional, into)] class: String,
  #[prop(optional, into)] href: String,
) -> impl IntoView {
  let class = tw_merge!(
    "
      focus:outline-none
      font-medium
      hover:text-foreground
      inline-flex
      items-center
      rounded-sm
      text-sm
      text-foreground/70
      transition-colors
    ",
    class
  );

  view! {
    <a data-name="NavigationMenuLink" href=href class=class>
      {children()}
    </a>
  }
}
