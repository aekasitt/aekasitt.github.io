/* ~~/build.rs */

// standard crates
use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::fs::{metadata, read_dir, read_to_string, write};
use std::path::{Path, PathBuf};
use std::process::Command;

// third-party crates
use charming::component::{Calendar, Title, VisualMap, VisualMapPiece, VisualMapType};
use charming::datatype::{DataFrame, DataPoint};
use charming::element::{CoordinateSystem, ItemStyle, Orient, Tooltip};
use charming::series::Heatmap;
use charming::{Chart, ImageRenderer};
use chrono::{DateTime, FixedOffset, Months, NaiveDate, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct ManifestEntry {
  created: NaiveDate,
  slug: String,
  title: String,
}

/// obtain markdown creation date via git log otherwise fallback on filesystem metadata
fn created_at(path: &PathBuf) -> NaiveDate {
  let git_out = Command::new("git")
    .args([
      "log",
      "--follow",
      "--format=%aI",
      "--diff-filter=A",
      "--",
      path.to_str().unwrap(),
    ])
    .output()
    .expect("git log failed");
  let iso = String::from_utf8_lossy(&git_out.stdout);
  let iso = iso.lines().last().unwrap_or("").trim().to_string();
  DateTime::parse_from_rfc3339(&iso)
    .map(|dt| dt.date_naive())
    .unwrap_or_else(|_| {
      let fallback: DateTime<Utc> = metadata(&path).unwrap().modified().unwrap().into();
      fallback.date_naive()
    })
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
      let created = created_at(&path);
      let raw = read_to_string(&path).expect("read markdown");
      let slug = stem.to_string();
      let title = raw
        .lines()
        .next()
        .map(|line| line.trim_start_matches("# ").trim().to_string())
        .filter(|line| !line.is_empty())
        .unwrap_or_else(|| "Untitled post".to_string());
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

  // Create a Contribution calendar
  let mut counts: HashMap<String, i64> = HashMap::new();
  for entry in &entries {
    *counts.entry(entry.created.to_string()).or_insert(0) += 1;
  }
  let heatmap: Vec<DataFrame> = counts
    .into_iter()
    .map(|(date, count)| vec![DataPoint::from(date), DataPoint::from(count)])
    .collect();
  let now = Utc::now();
  let chart = Chart::new()
    .calendar(
      Calendar::new()
        .item_style(ItemStyle::new().border_width(0.5))
        .range((
          now
            .checked_sub_months(Months::new(5))
            .expect("Resulting date out of range")
            .format("%Y-%m-%d")
            .to_string(),
          now.format("%Y-%m-%d").to_string(),
        ))
        .top(48),
    )
    .series(
      Heatmap::new()
        .coordinate_system(CoordinateSystem::Calendar)
        .data(heatmap),
    )
    .tooltip(Tooltip::new())
    .visual_map(
      VisualMap::new()
        .left("center")
        .orient(Orient::Horizontal)
        .pieces(vec![
          VisualMapPiece::new()
            .color("#c6e48b")
            .label("1")
            .max(1)
            .min(1),
          VisualMapPiece::new()
            .color("#7bc96f")
            .label("2")
            .max(2)
            .min(2),
          VisualMapPiece::new()
            .color("#239a3b")
            .label("3")
            .max(3)
            .min(3),
          VisualMapPiece::new()
            .color("#196127")
            .label("4+")
            .max(5)
            .min(4),
        ])
        .top(0)
        .type_(VisualMapType::Piecewise),
    );
  let mut renderer = ImageRenderer::new(600, 200);
  renderer.save(&chart, "assets/calendar.svg").unwrap();
  Ok(())
}
