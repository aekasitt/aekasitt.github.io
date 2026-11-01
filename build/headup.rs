/* ~~/build/headup.rs */

// standard library
use std::env;
use std::fs::{read_dir, write};
use std::path::Path;

// third-party crates
use chrono::{Datelike, Utc};

// local modules
use crate::models::{Entry, Headup};

/// Generate a dashboard summary after the markdown posts have been built.
pub fn compose_headup_recap(entries: &[Entry]) -> std::io::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(&manifest_dir).join("assets");
    let mut total_posts: usize = 0;
    for entry in read_dir(assets_dir.join("posts"))? {
        let entry = entry?;
        if entry.file_type()?.is_file()
            && entry.path().extension().and_then(|e| e.to_str()) == Some("md")
        {
            total_posts += 1;
        }
    }
    let generated_at = Utc::now();
    let today = generated_at.date_naive();
    let headup = Headup {
        generated_at,
        total_posts,
        first_published: entries.iter().map(|entry| entry.created).min(),
        latest_published: entries.iter().map(|entry| entry.created).max(),
        posts_this_month: entries
            .iter()
            .filter(|entry| {
                entry.created <= today
                    && entry.created.year() == today.year()
                    && entry.created.month() == today.month()
            })
            .count(),
        posts_this_year: entries
            .iter()
            .filter(|entry| entry.created <= today && entry.created.year() == today.year())
            .count(),
    };
    let mut line = serde_json::to_vec(&headup).map_err(std::io::Error::other)?;
    line.push(b'\n');
    write(assets_dir.join("headup.ndjson"), line)?;
    Ok(())
}
