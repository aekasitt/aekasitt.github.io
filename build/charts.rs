/* ~~/build/charts.rs */

// standard crates
use std::collections::HashMap;

// third-party crates
use charming::component::{
  Calendar, RadarCoordinate, RadarIndicator, VisualMap, VisualMapPiece, VisualMapType,
};
use charming::datatype::{DataFrame, DataPoint};
use charming::element::{AreaStyle, CoordinateSystem, ItemStyle, Orient, Tooltip};
use charming::series::{Heatmap, Radar};
use charming::{Chart, ImageRenderer};
use chrono::{DateTime, Months, Utc};

// local modules
use crate::models::{Entry, Tag};

/// Create a Contribution calendar
pub fn compile_contribution_calendar_chart(entries: Vec<Entry>) -> std::io::Result<()> {
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
  let mut renderer = ImageRenderer::new(600, 200);
  renderer.save(&chart, "assets/contributions.svg").unwrap();
  Ok(())
}

/// Compile breakdown Radar chart for Tags
pub fn compile_breakdown_radar_chart_for_tags(tags: Vec<Tag>) -> std::io::Result<()> {
  let mut breakdown = HashMap::new();
  for tag in tags {
    *breakdown.entry(tag).or_insert(0) += 1;
  }
  let max_value: i64 = *breakdown.values().max().unwrap_or(&0);
  let chart = Chart::new()
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
  let mut renderer = ImageRenderer::new(200, 180);
  renderer.save(&chart, "assets/tag-breakdown.svg").unwrap();
  Ok(())
}
