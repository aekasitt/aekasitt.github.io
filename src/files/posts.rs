/* ~~/src/files/posts.rs */

// third-party crates
use include_dir::{Dir, include_dir};
use leptos_router::static_routes::{StaticParamsMap, StaticRoute};

// local modules
use crate::models::Post;

static POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/posts");

pub fn get_post(slug: &str) -> Option<Post> {
  list_posts().into_iter().find(|post| post.slug == slug)
}

pub fn list_posts() -> Vec<Post> {
  let mut posts: Vec<Post> = POSTS_DIR
    .files()
    .filter_map(|file| {
      let path = file.path();
      if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
        return None;
      }
      let slug = path.file_stem()?.to_str()?.to_string();
      let markdown = file.contents_utf8()?;
      let (title, content) = parse_markdown_post(markdown);
      Some(Post {
        slug,
        title,
        content,
      })
    })
    .collect();

  posts.sort_by(|a, b| a.slug.cmp(&b.slug));
  posts
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

pub fn post_static_route() -> StaticRoute {
  StaticRoute::new().prerender_params(|| async {
    let mut params = StaticParamsMap::new();
    params.insert(
      "slug",
      list_posts().into_iter().map(|post| post.slug).collect(),
    );
    params
  })
}
