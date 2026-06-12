//! Mark the work done (deployed) and run the move-on checklist.

use crate::cli::Cli;
use crate::error::Result;
use crate::workspace::{self, Bet};
use colored::Colorize;

/// Run the `ship` command.
pub fn run(cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (path, bet) = workspace::require_active_bet(&ws)?;

    workspace::rewrite::<Bet, _>(&path, |b| b.status = "shipped".to_string())?;

    if !cli.quiet {
        println!("{} Bet {} shipped.", "🚢".bold(), bet.id);
        println!("  Move on — new requests are raw ideas for the next round, not interruptions.");
    }
    Ok(0)
}
