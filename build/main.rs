/* ~~/build/main.rs */

// standard library
#[cfg(debug_assertions)]
use std::fs::{File, create_dir};
#[cfg(debug_assertions)]
use std::io::Write;

// local modules
mod charts;
mod latest;
mod models;
mod ograph;

use charts::{compile_breakdown_radar_chart_for_tags, compile_contribution_calendar_chart};
use latest::capture_latest_notes_for_dashboard;
use ograph::render_opengraph_image;

/// build  hook
fn main() -> std::io::Result<()> {
  let (entries, tags) = capture_latest_notes_for_dashboard()?;
  let count = entries.len();
  let last_updated = entries
    .iter()
    .next()
    .ok_or(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Empty entries",
    ))?
    .created
    .to_string();
  compile_breakdown_radar_chart_for_tags(tags)?;
  compile_contribution_calendar_chart(entries)?;
  render_opengraph_image(count, &last_updated)?;
  #[cfg(debug_assertions)]
  {
    match create_dir("target/site") {
      Ok(_) => {}
      Err(_) => {}
    }
    match File::create("./target/site/index.html") {
      Ok(mut template) => write!(
        template,
        r#"<!DOCTYPE html>
<html lang='en'>
  <head>
    <link rel='icon' href='/favicon.ico' />
    <link rel='stylesheet' crossorigin href='/blockquotes.css' />
    <link rel='stylesheet' crossorigin href='/kanagawa.css' />
    <link rel='stylesheet' crossorigin href='/styles.css'>
    <meta charset='UTF-8' />
    <meta name='color-scheme' content='dark light' />
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
    <meta property='og:description' content='Collection of musings about Bitcoin, Python, Rust, Simplicity and WebAssembly by Aekasitt Guruvanich'>
    <meta property='og:image' content='/assets/opengraph.png' />
    <meta property='og:title' content="Guru's Gazette" />
    <meta property='og:type' content='website' />
    <meta property='og:url' content='http://localhost:3000' />
    <script src='https://unpkg.com/prismjs/prism.js'></script>
    <script src='https://unpkg.com/prismjs/components/prism-rust.min.js'></script>
    <script type='module'>
      import init, {{ hydrate }} from '/pkg/blog.js'
      await init('/pkg/blog.wasm')
      hydrate()
    </script>
    <title> Development template </title>
  </head>
  <body></body>
</html>
"#
      )?,
      Err(_) => println!("Unable to write file"),
    }
  };
  Ok(())
}
