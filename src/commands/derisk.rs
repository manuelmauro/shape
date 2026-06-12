//! Review a pitch's rabbit holes and no-gos — a focused risk read.

use crate::cli::{Cli, DeriskArgs};
use crate::error::Result;
use crate::workspace;
use colored::Colorize;

/// Run the `derisk` command.
pub fn run(args: &DeriskArgs, cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (path, pitch) = workspace::find_pitch(&ws, &args.name)?;
    let text = std::fs::read_to_string(&path)?;
    let body = workspace::split_frontmatter(&text)
        .map(|(_, b)| b)
        .unwrap_or(text.as_str());

    println!("{}", format!("Risk review — {}", pitch.title).bold());
    let mut found = false;
    for heading in ["Rabbit holes", "No-gos"] {
        if let Some(content) = workspace::section(body, heading) {
            found = true;
            println!("\n## {heading}\n\n{content}");
        }
    }
    if !found {
        println!("\n(no Rabbit holes or No-gos sections found)");
    }
    Ok(0)
}
