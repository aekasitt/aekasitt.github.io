/* ~~/src/files/posts.rs */

// third-party crates
use gloo_net::http::Request;
use nanoserde::{DeJson, DeJsonErr, DeJsonState, DeJsonTok};
use web_sys::RequestCache;

// local modules
use crate::models::Post;

// constants
pub const LATEST_ENTRIES_URL: &str = "/assets/latest.json";

#[derive(Clone, DeJson)]
pub struct Entry {
  pub banner: Option<String>,
  pub slug: String,
  pub title: String,
}

#[derive(Clone, DeJson)]
pub struct Latest {
  pub entries: Vec<Entry>,
  pub served_by: String,
}

pub async fn fetch_latest_entries() -> Result<Latest, String> {
  let response = Request::get(LATEST_ENTRIES_URL)
    .cache(RequestCache::NoCache)
    .send()
    .await
    .map_err(|e| e.to_string())?;
  if !response.ok() {
    return Err(format!(
      "request to retrieve latest entries failed with HTTP {}",
      response.status()
    ));
  }
  let headers = response.headers();
  let served_by = if let Some(server) = headers.get("server") {
    server
  } else {
    String::from("Development")
  };
  let text = response.text().await.map_err(|e| e.to_string())?;
  Ok(Latest {
    entries: DeJson::deserialize_json(&text).map_err(|e| e.to_string())?,
    served_by,
  })
}

pub async fn fetch_post(slug: &str) -> Result<Option<Post>, String> {
  let url = format!("/assets/posts/{slug}.md");
  let response = Request::get(&url).send().await.map_err(|e| e.to_string())?;
  if response.status() == 404 {
    return Ok(None);
  }
  if !response.ok() {
    return Err(format!(
      "post request failed with HTTP {}",
      response.status()
    ));
  }
  let markdown = response.text().await.map_err(|e| e.to_string())?;
  let (title, content) = parse_markdown_post(&markdown);
  Ok(Some(Post {
    content,
    slug: slug.to_string(),
    title,
  }))
}

pub fn parse_markdown_post(markdown: &str) -> (String, String) {
  let mut lines = markdown.lines();
  let title = lines
    .next()
    .map(|line| line.trim_start_matches("# ").trim().to_string())
    .filter(|line| !line.is_empty())
    .unwrap_or_else(|| "Untitled post".to_string());
  let content = lines.collect::<Vec<_>>().join("\n").trim().to_string();
  (title, content)
}
