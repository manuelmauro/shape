//! Accept a human gate: the first integrated slice, or the finished work.

use crate::cli::{AcceptArgs, Cli, Gate};
use crate::error::{Result, ShapeError};
use crate::workspace::{self, Bet};
use colored::Colorize;

/// Run the `accept` command.
pub fn run(args: &AcceptArgs, cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (path, bet) = workspace::require_active_bet(&ws)?;

    match args.gate {
        Gate::Slice => {
            workspace::rewrite::<Bet, _>(&path, |b| b.slice_accepted = true)?;
            if !cli.quiet {
                println!(
                    "{} First slice accepted for bet {}.",
                    "✓".green().bold(),
                    bet.id
                );
            }
        }
        Gate::Done => {
            // The gate has teeth: every scope must be done.
            let remaining = workspace::list_scopes(&ws, &bet.id)?
                .into_iter()
                .filter(|(_, scope)| scope.status != "done")
                .count();
            if remaining > 0 {
                return Err(ShapeError::ScopesIncomplete { count: remaining });
            }
            if !cli.quiet {
                println!(
                    "{} Done accepted for bet {} — run {}.",
                    "✓".green().bold(),
                    bet.id,
                    "shape ship".bold()
                );
            }
        }
    }
    Ok(0)
}
