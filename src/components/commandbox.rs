/* ~~/src/components/commandbox.rs */

// third-party crates
use leptos::html::Input;
use leptos::prelude::*;
use strum::Display;

// local crates
use crate::components::ui::command::{
  Command, CommandDescription, CommandFooter, CommandGroup, CommandGroupLabel, CommandHeader,
  CommandInput, CommandItemLink, CommandList, CommandTitle,
};
use crate::components::ui::input_group::{InputGroup, InputGroupAddon};
use crate::components::ui::kbd::Kbd;
use crate::icons::{
  ArrowDown, ArrowUp, ArrowUpRightFromSquare, Command as CommandIcon, CornerDownLeft, Marker,
  Search,
};

#[derive(Clone, Display)]
enum CommandCategory {
  External,
  Posts,
}

#[derive(Clone)]
struct CommandItemData {
  label: &'static str,
  href: &'static str,
  category: CommandCategory,
}
impl CommandItemData {
  fn icon(&self) -> AnyView {
    match self.category {
      CommandCategory::External => view! { <ArrowUpRightFromSquare /> }.into_any(),
      CommandCategory::Posts => view! { <Marker /> }.into_any(),
    }
  }
}

const EXTERNAL_ITEMS: &[CommandItemData] = &[
  CommandItemData {
    label: "Krutt",
    href: "https://krutt.github.io",
    category: CommandCategory::External,
  },
  CommandItemData {
    label: "Sponsor",
    href: "https://geyser.fund/project/gazette",
    category: CommandCategory::External,
  },
  CommandItemData {
    label: "Zines",
    href: "https://aekasitt.github.io/zines",
    category: CommandCategory::External,
  },
];

const POST_ITEMS: &[CommandItemData] = &[
  CommandItemData {
    label: "Accordion",
    href: "/components/accordion",
    category: CommandCategory::Posts,
  },
  CommandItemData {
    label: "Alert",
    href: "/components/alert",
    category: CommandCategory::Posts,
  },
  CommandItemData {
    label: "Alert Dialog",
    href: "/components/alert-dialog",
    category: CommandCategory::Posts,
  },
  CommandItemData {
    label: "Avatar",
    href: "/components/avatar",
    category: CommandCategory::Posts,
  },
  CommandItemData {
    label: "Badge",
    href: "/components/badge",
    category: CommandCategory::Posts,
  },
  CommandItemData {
    label: "Breadcrumb",
    href: "/components/breadcrumb",
    category: CommandCategory::Posts,
  },
];

#[component]
pub fn CommandBox(
  command_focused: RwSignal<bool>,
  search_toggled: ReadSignal<bool>,
) -> impl IntoView {
  view! {
    <div class=move || {
      if command_focused.get() || search_toggled.get() {
        "
          backdrop-blur-xs
          duration-200
          fixed
          flex
          inset-0
          items-start
          justify-center
          p-4
          pt-16
          shadow-lg
          transition-all
          z-50
        "
      } else {
        "hidden"
      }
    }>
      <Suspense fallback=move || view! { <div>"Loading commandbox..."</div> }>
        {move || Suspend::new(async move {
          LazyCommandBox(
            LazyCommandBoxProps::builder()
              .command_focused(command_focused)
              .search_toggled(search_toggled)
              .build()
            ).await
          })
        }
      </Suspense>
    </div>
  }
}

#[component]
#[lazy]
pub fn LazyCommandBox(
  command_focused: RwSignal<bool>,
  search_toggled: ReadSignal<bool>,
) -> AnyView {
  let command_input_ref = NodeRef::<Input>::new();
  Effect::new(move |_| {
    if search_toggled.get() {
      if let Some(element) = command_input_ref.get() {
        let _ = element.focus();
        command_focused.set(true);
      }
    }
  });
  view! {
    <div
      class="
        bg-popover
        border
        max-w-[450px]
        mx-auto
        my-6
        rounded-md
        w-full
      ">
      <CommandHeader>
        <CommandTitle>
          "Search blog..."
        </CommandTitle>
        <CommandDescription>
          "Search for a note from archive..."
        </CommandDescription>
      </CommandHeader>
      <Command>
        <InputGroup
          class="
            border-b
            h-9
            rounded-none
          ">
          <InputGroupAddon>
            <Search />
          </InputGroupAddon>
          <CommandInput
            attr:placeholder="Search blog..."
            class="
              border-0
              flex-1
              h-9
              py-0
              rounded-none
              shadow-none
            "
            node_ref=command_input_ref
            on:focus=move |_| command_focused.set(true)
            on:blur=move |_| command_focused.set(false)
          />
        </InputGroup>
        <CommandList
          attr:id="command_demo"
          attr:tabindex="-1"
          >
          {[(CommandCategory::External, EXTERNAL_ITEMS), (CommandCategory::Posts, POST_ITEMS)]
            .into_iter()
            .map(|(category, items)| {
              view! {
                <CommandGroup
                  attr:role="presentation"
                  class="p-0"
                  >
                  <CommandGroupLabel
                    attr:aria-hidden="true"
                    class="p-3"
                    >
                    {category.to_string()}
                  </CommandGroupLabel>
                  {items
                    .iter()
                    .map(|item| {
                      let icon = item.icon();
                      view! {
                        <CommandItemLink
                          attr:href=item.href
                          attr:rel="noopener noreferrer"
                          attr:target="_blank"
                          class="px-3"
                          >
                          {icon}
                          <span>
                            {item.label}
                          </span>
                        </CommandItemLink>
                      }
                    })
                    .collect::<Vec<_>>()}
                </CommandGroup>
              }
            })
            .collect::<Vec<_>>()}
        </CommandList>
      </Command>
      <CommandFooter>
        <div
          class="
            flex
            gap-2
            items-center
          ">
          <Kbd>
            <ArrowUp />
          </Kbd>
          <Kbd>
            <ArrowDown />
          </Kbd>
          <span>
            Navigate
          </span>
        </div>
        <div
          class="
            flex
            gap-2
            items-center
          ">
          <Kbd>
            <CornerDownLeft />
          </Kbd>
          <span>
            Go to Page
          </span>
        </div>
      </CommandFooter>
    </div>
  }
  .into_any()
}
