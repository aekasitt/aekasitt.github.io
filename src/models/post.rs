/* ~~/src/models/post.rs */

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
  pub slug: String,
  pub title: String,
  pub content: String,
}
