/* ~~/src/components/ui/kbd.rs */

// third-party crates
use leptos::prelude::*;
use leptos_ui::clx;

mod components {
  use super::*;
  clx! {
    Kbd,
    kbd,
    "
      bg-muted
      dark:[[data-slot=tooltip-content]_&]:bg-background/10
      font-medium
      font-sans
      gap-1
      h-5
      inline-flex
      items-center
      justify-center
      min-w-5
      pointer-events-none
      px-1
      rounded-sm
      select-none
      text-muted-foreground
      text-xs
      w-fit
      [&_svg:not([class*='size-'])]:size-3
      [[data-slot=tooltip-content]_&]:bg-background/20
      [[data-slot=tooltip-content]_&]:text-background
    "
  }
  clx! {
    KbdGroup,
    kbd,
    "
      gap-1
      inline-flex
      items-center
    "
  }
}
