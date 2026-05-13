/* ~~/src/components/statistics.rs */

// third-party crates
use leptos::html::Div;
use leptos::prelude::*;

// local crates
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Statistics(
  #[prop(optional, into)] count: usize,
  #[prop(optional, into)] updated: String,
) -> impl IntoView {
  let calendar = include_str!("../../assets/calendar.svg");
  let tagradar = include_str!("../../assets/tagradar.svg");
  let calendar_ref = NodeRef::<Div>::new();
  let tagradar_ref = NodeRef::<Div>::new();
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
    if let Some(element) = tagradar_ref.get() {
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
            <div
              class="h-auto w-full"
              inner_html=calendar
              node_ref=calendar_ref
              />
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardDescription>
              Tags
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div
              class="h-auto w-full"
              inner_html=tagradar
              node_ref=tagradar_ref
              />
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardDescription>
              Last updated
            </CardDescription>
            <CardTitle
              class="
                font-bold
                text-2xl
                text-right
              ">
              { updated }
            </CardTitle>
            <CardDescription>
              Total Posts
            </CardDescription>
            <CardTitle
              class="
                font-bold
                tabular-nums
                text-2xl
                text-right
              ">
              {count}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p
              class="
                text-muted-foreground
                text-sm
              ">
              In Numeris Veritas
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  }
}
