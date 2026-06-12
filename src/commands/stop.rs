//! Trip the circuit breaker manually and halt the active run.

use crate::cli::Cli;
use crate::error::Result;
use crate::workspace::{self, Bet};
use colored::Colorize;

/// Run the `stop` command.
pub fn run(cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (path, bet) = workspace::require_active_bet(&ws)?;

    workspace::rewrite::<Bet, _>(&path, |b| b.status = "stopped".to_string())?;

    if !cli.quiet {
        println!(
            "{} Circuit breaker tripped — bet {} stopped. Re-shape before betting again.",
            "⛔".red(),
            bet.id
        );
    }
    Ok(0)
}
