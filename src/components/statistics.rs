/* ~~/src/components/statistics.rs */

// third-party crates
use leptos::html::Div;
use leptos::prelude::*;

// local crates
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::card::{
  Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle,
};

#[component]
pub fn Statistics() -> impl IntoView {
  let calendar = include_str!("../../assets/calendar.svg");
  let calendar_ref = NodeRef::<Div>::new();
  Effect::new(move |_| {
    if let Some(element) = calendar_ref.get() {
      let children = element.children();
      for i in 0..children.length() {
        if let Some(child) = children.item(i) {
          let _ = child.remove_attribute("height");
          let _ = child.remove_attribute("width");
        }
      }
    }
  });
  view! {
    <div
      class="
        flex
        items-center
        justify-start
        mb-4
      ">
      <div
        class="
          gap-4
          grid
          grid-cols-2
          md:grid-cols-4
          w-full
        ">
        <Card
          class="
            col-span-2
          ">
          <CardHeader>
            <CardDescription>
              Post history
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div class="h-auto w-full" inner_html=calendar node_ref=calendar_ref/>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardDescription>
              Total Revenue
            </CardDescription>
            <CardTitle
              class="
                text-2xl
                font-bold
                tabular-nums
              ">
              $45,231.89
            </CardTitle>
            <CardAction>
              <Badge variant=BadgeVariant::Secondary>
                +12.5%
              </Badge>
            </CardAction>
          </CardHeader>
          <CardContent>
            <p
              class="
                text-muted-foreground
                text-sm
              ">
              +20.1% from last month
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardDescription>
              Active Users
            </CardDescription>
            <CardTitle
              class="
                font-bold
                tabular-nums
                text-2xl
              ">
              2,350
            </CardTitle>
            <CardAction>
              <Badge variant=BadgeVariant::Destructive>
                -3.2%
              </Badge>
            </CardAction>
          </CardHeader>
          <CardContent>
            <p
              class="
                text-muted-foreground
                text-sm
              ">
              -180 from last month
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  }
}
