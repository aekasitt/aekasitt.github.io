/* ~~/src/components/navigation.rs */

// third-party crates
use leptos::prelude::*;
use leptos_router::components::A;

// local modules
use crate::components::hooks::use_random::use_random_id;
use crate::components::search_box::SearchBox;
use crate::components::ui::navigation_menu::{
  NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuList,
  NavigationMenuTrigger,
};

#[component]
fn ListItem(
  #[prop(into)] href: String,
  #[prop(into)] title: String,
  children: Children,
) -> impl IntoView {
  view! {
    <li>
      <a
        href=href
        class="
          block
          focus:bg-accent
          focus:text-accent-foreground
          hover:bg-accent
          hover:text-accent-foreground
          leading-none
          no-underline
          p-3
          rounded-md
          select-none
          space-y-1
          transition-colors
          outline-none
        ">
        <div
          class="
            text-sm
            font-medium
            leading-none
          ">
          {title}
        </div>
        <p
          class="
            text-sm
            leading-snug
            line-clamp-2
            text-muted-foreground
          ">
          {children()}
        </p>
      </a>
    </li>
  }
}

#[component]
pub fn Navigation() -> impl IntoView {
  let id = use_random_id();
  view! {
    <div
      class="
        flex
        justify-center
        items-start
        py-8
      "
      id=id>
      <NavigationMenu>
        <NavigationMenuList>
          <NavigationMenuItem
            class="
              px-2
            ">
            <A href="/">
              <img
                alt="GitHub profile picture"
                class="
                  h-10
                  w-10
                "
                src="/assets/aekasitt.svg"
              />
            </A>
          </NavigationMenuItem>
          <NavigationMenuItem>
            <NavigationMenuTrigger>
              Author
            </NavigationMenuTrigger>
            <NavigationMenuContent>
              <ul
                class="
                  grid
                  gap-3
                  lg:w-[600px]
                  md:grid-cols-2
                  md:w-[500px]
                  p-0
                  w-[400px]
                ">
                <li class="row-span-3">
                  <a
                    class="
                      bg-gradient-to-b
                      flex
                      flex-col
                      focus:shadow-md
                      from-muted/50
                      h-full
                      hover:bg-accent
                      justify-end
                      no-underline
                      outline-none
                      p-6
                      w-full
                      rounded-md
                      select-none
                      to-muted
                    "
                    href="#"
                  >
                    <div
                      class="
                        mt-4
                        mb-2
                        text-lg
                        font-medium
                      ">
                      Sitt Guruvanich
                    </div>
                    <p
                      class="
                        leading-tight
                        text-muted-foreground
                        text-sm
                      ">
                      Rustacean, Patriarch, Bitcoin and
                      <strong>
                        krutt
                      </strong>
                    </p>
                  </a>
                </li>
                <ListItem href="#" title="Introduction">
                  This blog aims at providing the best bilingual Rust contents.
                </ListItem>
                <ListItem href="https://krutt.github.io" title="Krutt">
                  Visit Bitcoin-focused alternative by the same author.
                </ListItem>
                <ListItem href="https://geyser.fund/project/krutt" title="Sponsor">
                  Provide sponsorship and support for future works.
                </ListItem>
              </ul>
            </NavigationMenuContent>
          </NavigationMenuItem>
          <NavigationMenuItem>
            <NavigationMenuTrigger>
              Resources
            </NavigationMenuTrigger>
            <NavigationMenuContent>
              <ul
                class="
                  grid
                  gap-3
                  lg:w-[600px]
                  md:grid-cols-2
                  md:w-[500px]
                  p-0
                  w-[400px]
                ">
                <ListItem href="https://aekasitt.github.io/zines" title="Zines">
                  Bitcoin concepts condensed into printable A4 zines
                </ListItem>
                <ListItem href="https://pypi.org/project/aesir" title="Aesir">
                  Command Line Interface for setting up local Bitcoin regtest
                </ListItem>
                <ListItem href="https://github.com/aekasitt/libri" title="Libri">
                  Speed reader extension for your favorite web browser
                </ListItem>
                <ListItem href="https://pypi.org/project/fastapi-csrf-protect" title="FastAPI CSRF Protect">
                  FastAPI Extension providing protection against Cross-Site Request Forgery
                </ListItem>
                <ListItem href="https://aekasitt.github.io/notes" title="Notes">
                  Scrapbook for implementing ideas in Python programming language
                </ListItem>
                <ListItem href="https://krutt.github.io/thnbr" title="Thonburi">
                  Bitcoin Script enabled Scrapbook
                </ListItem>
              </ul>
            </NavigationMenuContent>
          </NavigationMenuItem>
          <NavigationMenuItem>
            <SearchBox/>
          </NavigationMenuItem>
        </NavigationMenuList>
      </NavigationMenu>
    </div>
  }
}
