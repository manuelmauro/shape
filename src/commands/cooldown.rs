//! Enter or leave the between-runs cool-down period.

use crate::cli::{Cli, CooldownAction, CooldownArgs};
use crate::error::Result;
use crate::workspace;
use colored::Colorize;

/// Run the `cooldown` command.
pub fn run(args: &CooldownArgs, cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let path = workspace::cooldown_path(&ws);

    match args.action {
        Some(CooldownAction::Start) => {
            std::fs::write(&path, "")?;
            if !cli.quiet {
                println!(
                    "{} Cool-down started — bug fixes, spikes, and re-shaping.",
                    "❄".cyan()
                );
            }
        }
        Some(CooldownAction::End) => {
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
            if !cli.quiet {
                println!("Cool-down ended.");
            }
        }
        None => {
            if path.exists() {
                println!("In cool-down.");
            } else {
                println!("Not in cool-down.");
            }
        }
    }
    Ok(0)
}
