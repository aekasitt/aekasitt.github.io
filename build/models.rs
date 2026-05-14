/* ~~/build/models.rs */

// third-party crates
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Entry {
  pub banner: Option<String>,
  pub created: NaiveDate,
  pub slug: String,
  pub tags: Option<Vec<Tag>>,
  pub title: String,
}

#[derive(Deserialize)]
pub struct Frontmatter {
  pub banner: Option<String>,
  pub tags: Option<Vec<Tag>>,
  pub title: String,
}

#[derive(Serialize)]
pub struct Manifest {
  pub entries: Vec<Entry>,
  pub updated: NaiveDate,
}

#[derive(Clone, Eq, Deserialize, Hash, PartialEq, Serialize)]
pub enum Tag {
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
