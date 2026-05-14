/* ~~/build.rs */

// standard crates
use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, metadata, read_dir, read_to_string, write};
use std::path::{Path, PathBuf};
use std::process::Command;

// third-party crates
use charming::component::{
  Calendar, RadarCoordinate, RadarIndicator, VisualMap, VisualMapPiece, VisualMapType,
};
use charming::datatype::{DataFrame, DataPoint};
use charming::element::{AreaStyle, CoordinateSystem, ItemStyle, Orient, Tooltip};
use charming::series::{Heatmap, Radar};
use charming::{Chart, ImageRenderer};
use chrono::{DateTime, Months, NaiveDate, Utc};
use markdown_frontmatter::parse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
struct Entry {
  banner: Option<String>,
  created: NaiveDate,
  slug: String,
  tags: Option<Vec<Tag>>,
  title: String,
}

#[derive(Deserialize)]
struct Frontmatter {
  banner: Option<String>,
  tags: Option<Vec<Tag>>,
  title: String,
}

#[derive(Serialize)]
struct Manifest {
  entries: Vec<Entry>,
  updated: NaiveDate,
}

#[derive(Clone, Eq, Deserialize, Hash, PartialEq, Serialize)]
enum Tag {
  #[serde(rename = "bitcoin")]
  Bitcoin,
  #[serde(rename = "guide")]
  Guide,
  #[serde(rename = "hype")]
  Hype,
  #[serde(rename = "meme")]
  Meme,
  #[serde(rename = "news")]
  News,
  #[serde(rename = "python")]
  Python,
  #[serde(rename = "rust")]
  Rust,
  #[serde(rename = "tidbit")]
  Tidbit,
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
  let posts_dir = Path::new(&manifest_dir).join("assets").join("posts");
  let posts_src = Path::new(&manifest_dir).join("posts");
  create_dir_all(&posts_dir).expect("create assets/posts directory");
  let mut entries: Vec<Entry> = Vec::new();
  let mut tags: Vec<Tag> = Vec::new();
  if posts_src.is_dir() {
    for entry in read_dir(&posts_src).expect("read posts") {
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
      let (frontmatter, content) = parse::<Frontmatter>(&raw).expect("Invalid Frontmatter");
      let post = posts_dir.join(format!("{slug}.md"));
      write(&post, content).expect("write content to destination");
      entries.push(Entry {
        banner: frontmatter.banner,
        created,
        slug,
        tags: frontmatter.tags.clone(),
        title: frontmatter.title,
      });
      tags.extend(frontmatter.tags.unwrap_or(vec![]));
    }
  }
  entries.sort_by_key(|item| Reverse(item.created));
  let manifest = Manifest {
    entries: entries.clone(),
    updated: Utc::now().date_naive(),
  };
  let json = serde_json::to_string(&manifest).expect("serialize manifest");
  write(assets_dir.join("manifest.json"), json).expect("write manifest.json");

  // Create a Contribution calendar
  let mut counts: HashMap<String, i64> = HashMap::new();
  for entry in &entries {
    *counts.entry(entry.created.to_string()).or_insert(0) += 1;
  }
  let contributions: Vec<DataFrame> = counts
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
        .data(contributions),
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

  // Create a Tags radar
  let mut breakdown = HashMap::new();
  for tag in tags {
    *breakdown.entry(tag).or_insert(0) += 1;
  }
  let max_value: i64 = *breakdown.values().max().unwrap_or(&0);
  let radar = Chart::new()
    .radar(
      RadarCoordinate::new()
        .indicator(vec![
          RadarIndicator::from(("Bitcoin", 0, max_value)),
          RadarIndicator::from(("Guide", 0, max_value)),
          RadarIndicator::from(("Hype", 0, max_value)),
          RadarIndicator::from(("Meme", 0, max_value)),
          RadarIndicator::from(("News", 0, max_value)),
          RadarIndicator::from(("Python", 0, max_value)),
          RadarIndicator::from(("Rust", 0, max_value)),
          RadarIndicator::from(("Tidbit", 0, max_value)),
        ])
        .radius("64%"),
    )
    .series(Radar::new().area_style(AreaStyle::new()).data(vec![(
      vec![
        breakdown.get(&Tag::Bitcoin).copied().unwrap_or(0),
        breakdown.get(&Tag::Guide).copied().unwrap_or(0),
        breakdown.get(&Tag::Hype).copied().unwrap_or(0),
        breakdown.get(&Tag::Meme).copied().unwrap_or(0),
        breakdown.get(&Tag::News).copied().unwrap_or(0),
        breakdown.get(&Tag::Python).copied().unwrap_or(0),
        breakdown.get(&Tag::Rust).copied().unwrap_or(0),
        breakdown.get(&Tag::Tidbit).copied().unwrap_or(0),
      ],
      "Tag breakdown",
    )]));
  let mut calendar_renderer = ImageRenderer::new(600, 200);
  calendar_renderer
    .save(&chart, "assets/calendar.svg")
    .unwrap();
  let mut tagradar_renderer = ImageRenderer::new(200, 180);
  tagradar_renderer
    .save(&radar, "assets/tagradar.svg")
    .unwrap();
  Ok(())
}
