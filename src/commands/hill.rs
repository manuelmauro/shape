//! Show or update the hill chart (uphill = figuring out, downhill = doing).

use crate::cli::{Cli, HillArgs, HillCommand, HillSetArgs};
use crate::error::{Result, ShapeError};
use crate::workspace::{self, Scope};
use chrono::Local;
use colored::Colorize;
use std::io::Write;

/// Run the `hill` command.
pub fn run(args: &HillArgs, cli: &Cli) -> Result<i32> {
    match &args.command {
        Some(HillCommand::Set(set_args)) => set(set_args, cli),
        None => render(cli),
    }
}

/// Update a scope's hill position and append a snapshot to the bet's log.
fn set(args: &HillSetArgs, cli: &Cli) -> Result<i32> {
    if args.position > 100 {
        return Err(ShapeError::InvalidHillPosition(args.position));
    }
    let ws = workspace::resolve(cli)?;
    let (_, bet) = workspace::require_active_bet(&ws)?;
    let (path, scope) = workspace::find_scope(&ws, &bet.id, &args.scope)?;

    workspace::rewrite::<Scope, _>(&path, |s| s.hill = args.position)?;

    let log = workspace::hill_log_path(&ws, &bet.id);
    if let Some(parent) = log.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let entry = serde_json::json!({
        "time": Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        "scope": scope.name,
        "position": args.position,
    });
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log)?;
    writeln!(file, "{entry}")?;

    if !cli.quiet {
        println!(
            "{} {} → hill {} ({})",
            "✓".green().bold(),
            scope.name,
            args.position,
            workspace::hill_phase(args.position)
        );
    }
    Ok(0)
}

/// Render the active bet's scopes as an ASCII hill.
fn render(cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (_, bet) = workspace::require_active_bet(&ws)?;
    let scopes = workspace::list_scopes(&ws, &bet.id)?;

    println!(
        "{}",
        format!("hill — bet {} ({})", bet.id, bet.pitch).bold()
    );
    println!();
    if scopes.is_empty() {
        println!("  (no scopes yet)");
        return Ok(0);
    }
    let width = scopes.iter().map(|(_, s)| s.name.len()).max().unwrap_or(0);
    for (_, scope) in &scopes {
        println!(
            "  {:width$}  [{}] {:>3}  {}",
            scope.name,
            track(scope.hill),
            scope.hill,
            workspace::hill_phase(scope.hill),
            width = width,
        );
    }
    println!("\n  ● position · left half = uphill (figuring out) · right half = downhill (doing)");
    Ok(0)
}

/// A 20-cell track with a `●` at the scaled position.
fn track(position: u8) -> String {
    const WIDTH: usize = 20;
    let idx = (position as usize * (WIDTH - 1)) / 100;
    (0..WIDTH)
        .map(|i| if i == idx { '●' } else { '·' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_places_marker_at_scaled_position() {
        assert!(track(0).starts_with('●'));
        assert!(track(100).ends_with('●'));
        assert_eq!(track(50).chars().filter(|&c| c == '●').count(), 1);
    }
}
