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
  let contributions = include_str!("../../assets/contributions.svg");
  let tag_breakdown = include_str!("../../assets/tag-breakdown.svg");
  let contributions_ref = NodeRef::<Div>::new();
  let tag_breakdown_ref = NodeRef::<Div>::new();
  Effect::new(move |_| {
    if let Some(element) = contributions_ref.get() {
      let children = element.children();
      for i in 0..children.length() {
        if let Some(child) = children.item(i) {
          let _ = child.remove_attribute("height");
          let _ = child.remove_attribute("width");
        }
      }
    }
    if let Some(element) = tag_breakdown_ref.get() {
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
              inner_html=contributions
              node_ref=contributions_ref
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
              inner_html=tag_breakdown
              node_ref=tag_breakdown_ref
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
