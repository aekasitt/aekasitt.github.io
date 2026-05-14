/* ~~/build/latest.rs */

// standard library
use std::cmp::Reverse;
use std::env;
use std::fs::{create_dir_all, metadata, read_dir, read_to_string, write};
use std::path::{Path, PathBuf};
use std::process::Command;

// third-party crates
use chrono::{DateTime, NaiveDate, Utc};
use markdown_frontmatter::parse;

// local modules
use crate::models::{Entry, Frontmatter, Manifest, Tag};

pub fn capture_latest_notes_for_dashboard() -> std::io::Result<(Vec<Entry>, Vec<Tag>)> {
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
  Ok((entries, tags))
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
