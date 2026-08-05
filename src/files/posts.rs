/* ~~/src/files/posts.rs */

// third-party crates
use futures_util::StreamExt;
use gloo_net::http::Request;
use js_sys::{Reflect, Uint8Array};
use nanoserde::DeJson;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStreamDefaultReader, RequestCache, Response, TextDecoder};

// local modules
use crate::models::Post;

// constants
const LATEST_ENTRY_JSON: &str = "/assets/latest.ndjson";
const MAX_ALLOWED_BYTES: usize = 500 * 1024;

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
  let response = Request::get(LATEST_ENTRY_JSON)
    .header("Range", &format!("bytes=0-{}", MAX_ALLOWED_BYTES))
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
  let web_sys_response: web_sys::Response = response.into();
  let stream = web_sys_response
    .body()
    .ok_or_else(|| "response has no body stream".to_string())?;
  let reader = stream
    .get_reader()
    .dyn_into::<ReadableStreamDefaultReader>()
    .map_err(|e| format!("failed to cast reader: {:?}", e))?;
  let mut buffer = String::new();
  let mut entries: Vec<Entry> = vec![];
  let decoder = TextDecoder::new_with_label("utf-8")
    .map_err(|e| format!("failed to create decoder: {:?}", e))?;
  loop {
    let read_result = JsFuture::from(reader.read())
      .await
      .map_err(|e| format!("stream read error: {:?}", e))?;
    let done = Reflect::get(&read_result, &"done".into())
      .map_err(|e| format!("failed to read 'done' property: {:?}", e))?
      .as_bool()
      .unwrap_or(true);
    if done {
      break;
    }
    let chunk = Reflect::get(&read_result, &"value".into())
      .map_err(|e| format!("failed to read 'value' property: {:?}", e))?;
    let bytes = Uint8Array::new(&chunk);
    if let Ok(text) = decoder.decode_with_buffer_source(&bytes) {
      buffer.push_str(&text);
      while let Some(pos) = buffer.find('\n') {
        let line: String = buffer.drain(..=pos).collect();
        let trimmed = line.trim();
        if !trimmed.is_empty()
          && let Ok(entry) = DeJson::deserialize_json(&trimmed)
        {
          entries.push(entry);
        }
      }
    }
  }
  Ok(Latest { entries, served_by })
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
