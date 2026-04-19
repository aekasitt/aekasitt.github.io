/* ~~/build.rs */

// standard crates
use std::env;
use std::fs::{copy, create_dir_all, read_dir, read_to_string, write};
use std::path::Path;

// third-party crates
use serde::Serialize;

#[derive(Serialize)]
struct ManifestEntry {
  slug: String,
  title: String,
}

/// build  hook
fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
  let out_dir = Path::new(&manifest_dir).join("assets").join("posts");
  let posts_src = Path::new(&manifest_dir).join("posts");
  create_dir_all(&out_dir).expect("create assets/posts");
  let mut entries: Vec<ManifestEntry> = Vec::new();
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
      let slug = stem.to_string();
      let raw = read_to_string(&path).expect("read markdown");
      let title = raw
        .lines()
        .next()
        .map(|line| line.trim_start_matches("# ").trim().to_string())
        .filter(|line| !line.is_empty())
        .unwrap_or_else(|| "Untitled post".to_string());
      let dest = out_dir.join(format!("{slug}.md"));
      copy(&path, &dest).expect("copy markdown into assets");
      entries.push(ManifestEntry { slug, title });
    }
  }
  entries.sort_by(|a, b| a.slug.cmp(&b.slug));
  let json = serde_json::to_string(&entries).expect("serialize manifest");
  write(out_dir.join("manifest.json"), json).expect("write manifest.json");
}
