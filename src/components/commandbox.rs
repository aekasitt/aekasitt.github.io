/* ~~/src/components/commandbox.rs */

// third-party crates
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use leptos::html::Input;
use leptos::prelude::*;
use strum::Display;

// local crates
use crate::components::ui::command::{
  Command, CommandDescription, CommandEmpty, CommandFooter, CommandGroup, CommandGroupLabel,
  CommandHeader, CommandInput, CommandItemLink, CommandList, CommandTitle,
};
use crate::components::ui::input_group::{InputGroup, InputGroupAddon};
use crate::components::ui::kbd::Kbd;
use crate::files::posts::{SearchEntry, fetch_search_entries};
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

fn rank_posts(entries: &[SearchEntry], query: &str) -> Vec<SearchEntry> {
  let query = query.trim();
  if query.is_empty() {
    return entries.iter().take(8).cloned().collect();
  }

  let matcher = SkimMatcherV2::default().ignore_case();
  let query_lower = query.to_lowercase();
  let mut matches = entries
    .iter()
    .filter_map(|entry| {
      let title_score = matcher
        .fuzzy_match(&entry.title, query)
        .map(|score| score + 1_000);
      let slug_score = matcher
        .fuzzy_match(&entry.slug, query)
        .map(|score| score + 250);
      let tags = entry.tags.as_deref().unwrap_or_default().join(" ");
      let tag_score = matcher.fuzzy_match(&tags, query).map(|score| score + 500);
      let prefix_bonus = entry
        .title
        .to_lowercase()
        .starts_with(&query_lower)
        .then_some(1_000)
        .unwrap_or_default();
      [title_score, slug_score, tag_score]
        .into_iter()
        .flatten()
        .max()
        .map(|score| (score + prefix_bonus, entry.clone()))
    })
    .collect::<Vec<_>>();
  matches.sort_by(|(left_score, left), (right_score, right)| {
    right_score
      .cmp(left_score)
      .then_with(|| right.created.cmp(&left.created))
  });
  matches
    .into_iter()
    .take(20)
    .map(|(_, entry)| entry)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn entry(created: &str, slug: &str, tags: &[&str], title: &str) -> SearchEntry {
    SearchEntry {
      created: created.to_string(),
      slug: slug.to_string(),
      tags: Some(tags.iter().map(|tag| tag.to_string()).collect()),
      title: title.to_string(),
    }
  }

  #[test]
  fn title_matches_rank_above_tag_matches() {
    let entries = vec![
      entry("2026-09-28", "daily-note", &["rust"], "Daily note"),
      entry("2026-09-27", "rust-guide", &["guide"], "Rust guide"),
    ];

    let matches = rank_posts(&entries, "rust");

    assert_eq!(matches[0].slug, "rust-guide");
    assert_eq!(matches[1].slug, "daily-note");
  }

  #[test]
  fn skim_matching_handles_non_contiguous_queries() {
    let entries = vec![entry(
      "2026-09-28",
      "rust-cancellation-primer",
      &["rust"],
      "Async cancellation primer",
    )];

    let matches = rank_posts(&entries, "acp");

    assert_eq!(matches[0].slug, "rust-cancellation-primer");
  }
}

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
  let query = RwSignal::new(String::new());
  let search_entries = LocalResource::new(|| async move { fetch_search_entries().await });
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
      <Command should_filter=false>
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
            on_search_change=Callback::new(move |value| query.set(value))
            on:focus=move |_| command_focused.set(true)
            on:blur=move |_| command_focused.set(false)
          />
        </InputGroup>
        <CommandList
          attr:id="command_demo"
          attr:tabindex="-1"
          >
          <CommandGroup
            attr:role="presentation"
            class="p-0"
            style:display=move || if query.get().trim().is_empty() { "block" } else { "none" }
            >
            <CommandGroupLabel attr:aria-hidden="true" class="p-3">
              {CommandCategory::External.to_string()}
            </CommandGroupLabel>
            {EXTERNAL_ITEMS.iter().map(|item| {
              let icon = item.icon();
              view! {
                <CommandItemLink
                  attr:href=item.href
                  attr:rel="noopener noreferrer"
                  attr:target="_blank"
                  class="px-3"
                  >
                  {icon}
                  <span>{item.label}</span>
                </CommandItemLink>
              }
            }).collect::<Vec<_>>()}
          </CommandGroup>
          <CommandGroup attr:role="presentation" class="p-0">
            <CommandGroupLabel attr:aria-hidden="true" class="p-3">
              {CommandCategory::Posts.to_string()}
            </CommandGroupLabel>
            <Suspense fallback=move || view! { <CommandEmpty>"Loading posts…"</CommandEmpty> }>
              {move || match search_entries.get() {
                None => view! { <CommandEmpty>"Loading posts…"</CommandEmpty> }.into_any(),
                Some(Err(err)) => view! {
                  <CommandEmpty>"Could not load search index: " {err}</CommandEmpty>
                }.into_any(),
                Some(Ok(entries)) => {
                  let matches = rank_posts(&entries, &query.get());
                  if matches.is_empty() {
                    view! { <CommandEmpty>"No matching posts."</CommandEmpty> }.into_any()
                  } else {
                    matches.into_iter().map(|entry| view! {
                      <CommandItemLink
                        attr:href=format!("/post/{}/", entry.slug)
                        class="px-3"
                        >
                        <Marker />
                        <span class="truncate">{entry.title}</span>
                      </CommandItemLink>
                    }).collect::<Vec<_>>().into_any()
                  }
                },
              }}
            </Suspense>
          </CommandGroup>
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
