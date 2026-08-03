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
  ArrowDown, ArrowRight, ArrowUp, CircleDashed, Command as CommandIcon, CornerDownLeft, Search,
};

#[derive(Clone, Debug, Display)]
enum CommandCategory {
  Pages,
  Components,
}

#[derive(Clone, Debug)]
struct CommandItemData {
  label: &'static str,
  href: &'static str,
  category: CommandCategory,
}

const PAGES_ITEMS: &[CommandItemData] = &[
  CommandItemData {
    label: "Docs",
    href: "/docs",
    category: CommandCategory::Pages,
  },
  CommandItemData {
    label: "Components",
    href: "/components",
    category: CommandCategory::Pages,
  },
  CommandItemData {
    label: "Blocks",
    href: "/blocks",
    category: CommandCategory::Pages,
  },
];

impl CommandItemData {
  fn icon(&self) -> AnyView {
    match self.category {
      CommandCategory::Pages => view! { <ArrowRight /> }.into_any(),
      CommandCategory::Components => view! { <CircleDashed /> }.into_any(),
    }
  }
}

const COMPONENTS_ITEMS: &[CommandItemData] = &[
  CommandItemData {
    label: "Accordion",
    href: "/components/accordion",
    category: CommandCategory::Components,
  },
  CommandItemData {
    label: "Alert",
    href: "/components/alert",
    category: CommandCategory::Components,
  },
  CommandItemData {
    label: "Alert Dialog",
    href: "/components/alert-dialog",
    category: CommandCategory::Components,
  },
  CommandItemData {
    label: "Avatar",
    href: "/components/avatar",
    category: CommandCategory::Components,
  },
  CommandItemData {
    label: "Badge",
    href: "/components/badge",
    category: CommandCategory::Components,
  },
  CommandItemData {
    label: "Breadcrumb",
    href: "/components/breadcrumb",
    category: CommandCategory::Components,
  },
];

#[component]
pub fn CommandBox(
  command_focused: RwSignal<bool>,
  search_toggled: ReadSignal<bool>,
) -> impl IntoView {
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
        <CommandList attr:id="command_demo" attr:tabindex="-1">
          {[(CommandCategory::Pages, PAGES_ITEMS), (CommandCategory::Components, COMPONENTS_ITEMS)]
            .into_iter()
            .map(|(category, items)| {
              view! {
                <CommandGroup attr:role="presentation" class="p-0">
                  <CommandGroupLabel attr:aria-hidden="true" class="p-3">
                    {category.to_string()}
                  </CommandGroupLabel>
                  {items
                    .iter()
                    .map(|item| {
                      let icon = item.icon();
                      view! {
                        <CommandItemLink
                          class="px-3"
                          attr:href=item.href
                          attr:target="_blank"
                          attr:rel="noopener noreferrer"
                        >
                          {icon}
                          <span>{item.label}</span>
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
        <div class="flex gap-2 items-center">
          <Kbd>
            <ArrowUp />
          </Kbd>
          <Kbd>
            <ArrowDown />
          </Kbd>
          <span>Navigate</span>
        </div>
        <div class="flex gap-2 items-center">
          <Kbd>
            <CornerDownLeft />
          </Kbd>
          <span>Go to Page</span>
        </div>
      </CommandFooter>
    </div>
  }
}
