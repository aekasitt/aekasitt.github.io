/* ~~/src/components/statistics.rs */

// third-party crates
use leptos::prelude::*;

// local crates
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::card::{
  Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle,
};

#[component]
pub fn Statistics() -> impl IntoView {
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
          max-w-2xl
          md:grid-cols-4
        ">
        <Card
          class="
            col-span-2
          ">
          <CardHeader>
            Post history
          </CardHeader>
          <CardContent>
            <iframe src="/assets/calendar.html"></iframe>
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
