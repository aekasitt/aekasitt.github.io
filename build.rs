/* ~~/build.rs */

// standard crates
use std::cmp::Reverse;
use std::env;
use std::fs::{metadata, read_dir, read_to_string, write};
use std::path::Path;

// third-party crates
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct ManifestEntry {
  created: DateTime<Utc>,
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
      let created = metadata(path)?.created()?.into();
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
  Ok(())
}
