/* ~~/build/main.rs */

// local modules
mod charts;
mod latest;
mod models;

use charts::{compile_breakdown_radar_chart_for_tags, compile_contribution_calendar_chart};
use latest::capture_latest_notes_for_dashboard;

/// build  hook
fn main() -> std::io::Result<()> {
  let (entries, tags) = capture_latest_notes_for_dashboard()?;
  compile_breakdown_radar_chart_for_tags(tags)?;
  compile_contribution_calendar_chart(entries)?;
  Ok(())
}
