/* ~~/src/files/posts.rs */

// third-party crates
use gloo_net::http::Request;
use serde::Deserialize;

// local modules
use crate::models::Post;

/// Same path layout as `cargo-leptos` output under `site-root` (`target/site/posts/`).
pub const POSTS_MANIFEST_URL: &str = "/posts/manifest.json";

#[derive(Clone, Debug, Deserialize)]
pub struct PostSummary {
  pub slug: String,
  pub title: String,
}

pub fn post_markdown_url(slug: &str) -> String {
  format!("/posts/{slug}.md")
}

pub async fn fetch_post_summaries() -> Result<Vec<PostSummary>, String> {
  let response = Request::get(POSTS_MANIFEST_URL)
    .send()
    .await
    .map_err(|e| e.to_string())?;
  if !response.ok() {
    return Err(format!(
      "manifest request failed with HTTP {}",
      response.status()
    ));
  }
  let text = response.text().await.map_err(|e| e.to_string())?;
  serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub async fn fetch_post(slug: &str) -> Result<Option<Post>, String> {
  let url = post_markdown_url(slug);
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
    slug: slug.to_string(),
    title,
    content,
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
