/* ~~/src/components/nav_bar.rs */

// third-party crates
use leptos::prelude::*;

// local modules
use crate::components::hooks::use_random::use_random_id;
use crate::components::search_box::SearchBox;
use crate::components::ui::navigation_menu::{
  NavigationMenu, NavigationMenuContent, NavigationMenuItem,
  NavigationMenuList, NavigationMenuTrigger,
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
pub fn NavBar() -> impl IntoView {
  let id = use_random_id();
  view! {
    <div
      class="
        flex
        justify-center
        items-start
        py-8
        min-h-[350px]
      "
      id=id>
      <img alt="Krutt" class="h-10 w-10" src="assets/krutt.svg"/>
      <NavigationMenu>
        <NavigationMenuList>
          <NavigationMenuItem>
            <NavigationMenuTrigger>"Getting Started"</NavigationMenuTrigger>
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
                      "rust/ui"
                    </div>
                    <p
                      class="
                        leading-tight
                        text-muted-foreground
                        text-sm
                      ">
                      "Beautifully designed components built with Leptos and Tailwind CSS."
                    </p>
                  </a>
                </li>
                <ListItem href="#" title="Introduction">
                  "Re-usable components built using Leptos and Tailwind CSS."
                </ListItem>
                <ListItem href="#" title="Installation">
                  "How to install dependencies and structure your app."
                </ListItem>
                <ListItem href="#" title="Typography">
                  "Styles for headings, paragraphs, lists and more."
                </ListItem>
              </ul>
            </NavigationMenuContent>
          </NavigationMenuItem>
          <NavigationMenuItem>
            <NavigationMenuTrigger>"Components"</NavigationMenuTrigger>
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
                <ListItem href="#" title="Alert">
                  "Displays a callout for user attention."
                </ListItem>
                <ListItem href="#" title="Alert Dialog">
                  "A modal dialog that interrupts the user."
                </ListItem>
                <ListItem href="#" title="Button">
                  "Triggers an action or event."
                </ListItem>
                <ListItem href="#" title="Card">
                  "Displays content in a card container."
                </ListItem>
                <ListItem href="#" title="Input">
                  "Displays a form input field."
                </ListItem>
                <ListItem href="#" title="Select">
                  "Displays a list of options to pick from."
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
