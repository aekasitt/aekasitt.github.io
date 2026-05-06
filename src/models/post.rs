/* ~~/src/models/post.rs */

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Post {
  pub content: String,
  pub slug: String,
  pub title: String,
}
