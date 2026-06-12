//! Show the run dashboard: active bet, budget, scopes, and hill positions.

use crate::cli::Cli;
use crate::error::Result;
use crate::workspace;
use colored::Colorize;

/// Run the `status` command.
pub fn run(cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;

    if workspace::cooldown_path(&ws).exists() {
        println!("{}  (no scheduled work)\n", "❄ cool-down".cyan());
    }

    let Some((_, bet)) = workspace::active_bet(&ws)? else {
        let pitches = workspace::list_pitches(&ws)?;
        let shaped = pitches.iter().filter(|(_, p)| p.status == "shaped").count();
        println!("No active bet.");
        println!(
            "Pitches: {shaped} shaped, {} shaping",
            pitches.len() - shaped
        );
        if shaped > 0 {
            println!("Place a bet: {}", "shape bet <pitch>".bold());
        }
        return Ok(0);
    };

    let scopes = workspace::list_scopes(&ws, &bet.id)?;
    let done = scopes.iter().filter(|(_, s)| s.status == "done").count();

    println!(
        "{}",
        format!("Active bet {} · {} ({})", bet.id, bet.pitch, bet.appetite).bold()
    );
    println!("Status: {}", bet.status);
    println!(
        "Budget: {} tokens · {} iterations · {} min",
        bet.budget.tokens, bet.budget.iterations, bet.budget.minutes
    );
    println!(
        "Slice gate: {}",
        if bet.slice_accepted {
            "accepted"
        } else {
            "not yet accepted"
        }
    );

    println!("\nScopes ({done}/{} done):", scopes.len());
    if scopes.is_empty() {
        println!("  (none yet — `shape scope add <name>`)");
    }
    for (_, scope) in &scopes {
        let phase = workspace::hill_phase(scope.hill);
        let marker = if scope.status == "done" { "✓" } else { "•" };
        println!("  {marker} {:>3}  {:9}{}", scope.hill, phase, scope.name);
    }
    Ok(0)
}
