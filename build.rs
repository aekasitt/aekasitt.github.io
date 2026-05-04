/* ~~/build.rs */

// standard crates
use std::cmp::Reverse;
use std::env;
use std::fs::{metadata, read_dir, read_to_string, write};
use std::path::Path;

// third-party crates
use charming::Chart;
use charming::HtmlRenderer;
use charming::component::{Calendar, Title, VisualMap, VisualMapType};
use charming::datatype::DataFrame;
use charming::element::{CoordinateSystem, ItemStyle, Orient, Tooltip};
use charming::series::Heatmap;
use chrono::{DateTime, Months, NaiveDate, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct ManifestEntry {
  created: NaiveDate,
  slug: String,
  title: String,
}

/// build  hook
fn main() -> std::io::Result<()> {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
  let assets_dir = Path::new(&manifest_dir).join("assets");
  let posts_dir = Path::new(&manifest_dir).join("posts");
  let mut entries: Vec<ManifestEntry> = Vec::new();
  if posts_dir.is_dir() {
    for entry in read_dir(&posts_dir).expect("read posts") {
      let entry = entry.expect("dir entry");
      let path = entry.path();
      if path.extension().and_then(|e| e.to_str()) != Some("md") {
        continue;
      }
      let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
        continue;
      };
      let slug = stem.to_string();
      let raw = read_to_string(&path).expect("read markdown");
      let title = raw
        .lines()
        .next()
        .map(|line| line.trim_start_matches("# ").trim().to_string())
        .filter(|line| !line.is_empty())
        .unwrap_or_else(|| "Untitled post".to_string());
      let created_datetime: DateTime<Utc> = metadata(path)?.created()?.into();
      let created = created_datetime.date_naive();
      entries.push(ManifestEntry {
        created,
        slug,
        title,
      });
    }
  }
  entries.sort_by_key(|item| Reverse(item.created));
  let json = serde_json::to_string(&entries).expect("serialize manifest");
  write(assets_dir.join("manifest.json"), json).expect("write manifest.json");

  // Create a Heatmap calendar for landing page
  let mut heatmap: Vec<DataFrame> = Vec::with_capacity(151);
  for entry in entries {
    heatmap.push(vec![entry.created.to_string().into()]);
  }
  let now = Utc::now();
  let chart = Chart::new()
    .calendar(
      Calendar::new()
        .item_style(ItemStyle::new().border_width(0.5))
        .range((
          now
            .checked_sub_months(Months::new(3))
            .expect("Resulting date out of range")
            .format("%Y-%m-%d")
            .to_string(),
          now.format("%Y-%m-%d").to_string(),
        ))
        .top(120),
    )
    .series(
      Heatmap::new()
        .coordinate_system(CoordinateSystem::Calendar)
        .data(heatmap),
    )
    .tooltip(Tooltip::new())
    .title(Title::new().top(30).left("center").text("Daily Posts"))
    .visual_map(
      VisualMap::new()
        .left("center")
        .max(10000)
        .min(0)
        .orient(Orient::Horizontal)
        .top(65)
        .type_(VisualMapType::Piecewise),
    );
  let mut renderer = HtmlRenderer::new("Calendar", 1000, 800);
  renderer.save(&chart, "assets/calendar.html").unwrap();
  Ok(())
}
