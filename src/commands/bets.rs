//! List active and past bets.

use crate::cli::Cli;
use crate::error::Result;
use crate::workspace;

/// Run the `bets` command. One line per bet: `ID STATUS APPETITE PITCH`.
pub fn run(cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    for (_, bet) in workspace::list_bets(&ws)? {
        println!(
            "{:5}{:10}{:7}{}",
            bet.id, bet.status, bet.appetite, bet.pitch
        );
    }
    Ok(0)
}
