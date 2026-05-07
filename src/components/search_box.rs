/* ~~/src/components/search_box.rs */

// third-party crates
use leptos::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

// local crates
use crate::components::ui::command::{
  Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
};
use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};
use crate::icons::{Command as CommandIcon, Search};

#[derive(Clone, Copy, Display, EnumIter, Eq, PartialEq)]
enum Content {
  Project,
  Spotlight,
  Tutorial,
}

#[component]
pub fn SearchBox() -> impl IntoView {
  let value_signal = RwSignal::new(None::<Content>);

  view! {
    <Popover align=PopoverAlign::Start>
      <PopoverTrigger class="justify-between w-[200px]">
        <span class="truncate">
          {move || value_signal.get().map(|l| l.to_string()).unwrap_or_else(|| "Search".into())}
        </span>
        <CommandIcon class="ml-auto opacity-50 size-4" />
      </PopoverTrigger>
      <PopoverContent class="p-0 w-[200px]">
        <Command>
          <div class="flex gap-2 items-center px-2 border-b">
            <Search class="size-4 text-muted-foreground shrink-0" />
            <CommandInput attr:placeholder="Search content..." />
          </div>
          <CommandList>
            <CommandEmpty>"No content found."</CommandEmpty>
            <CommandGroup>
              {Content::iter()
                .map(|language| {
                  let label = language.to_string();
                  let is_selected = Signal::derive(move || value_signal.get() == Some(language));
                  view! {
                    <CommandItem
                      value=label.clone()
                      selected=is_selected
                      on_select=Callback::new(move |_| {
                          value_signal.set(Some(language));
                      })
                      >
                      {label}
                    </CommandItem>
                  }
                })
                .collect_view()}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  }
}
