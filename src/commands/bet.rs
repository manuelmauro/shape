//! Place a bet on a shaped pitch and arm the circuit breaker (human gate).

use crate::cli::{Appetite, BetArgs, Cli};
use crate::error::{Result, ShapeError};
use crate::workspace::{self, Bet, Budget};
use chrono::Local;
use colored::Colorize;

/// Run the `bet` command.
pub fn run(args: &BetArgs, cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (_, pitch) = workspace::find_pitch(&ws, &args.name)?;

    // Gate: only shaped pitches can be bet on, and only one bet at a time.
    if pitch.status != "shaped" {
        return Err(ShapeError::PitchNotShaped { name: pitch.name });
    }
    if let Some((_, active)) = workspace::active_bet(&ws)? {
        return Err(ShapeError::ActiveBetExists { id: active.id });
    }

    let appetite = args.appetite.unwrap_or(if pitch.appetite == "big" {
        Appetite::Big
    } else {
        Appetite::Small
    });
    let mut budget = Budget::for_appetite(appetite);
    if let Some(tokens) = args.tokens {
        budget.tokens = tokens;
    }
    if let Some(iterations) = args.iterations {
        budget.iterations = iterations;
    }
    if let Some(minutes) = args.minutes {
        budget.minutes = minutes;
    }

    let id = workspace::next_bet_id(&ws)?;
    let bet = Bet {
        id: id.clone(),
        pitch: pitch.name.clone(),
        appetite: appetite.as_str().to_string(),
        status: "active".to_string(),
        placed: Local::now().date_naive().format("%Y-%m-%d").to_string(),
        budget: budget.clone(),
        slice_accepted: false,
    };

    let dir = ws.join(workspace::BET_DIR);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{id}-{}.md", pitch.name));
    let body = format!(
        "
# Bet {id} — {title}

Pitch: `{pitch}` ({appetite})
Budget: {tokens} tokens · {iterations} iterations · {minutes} min

Run `shape build` to start the run, then map scopes with `shape scope add <name>`.
",
        title = pitch.title,
        pitch = pitch.name,
        appetite = appetite.as_str(),
        tokens = budget.tokens,
        iterations = budget.iterations,
        minutes = budget.minutes,
    );
    std::fs::write(&path, workspace::compose(&bet, &body))?;

    if !cli.quiet {
        println!(
            "{} Placed bet {} on '{}' ({})",
            "✓".green().bold(),
            id,
            pitch.name,
            appetite.as_str()
        );
        println!(
            "  Circuit breaker armed: {} tokens · {} iterations · {} min",
            budget.tokens, budget.iterations, budget.minutes
        );
        println!("  Next: {}", "shape build".bold());
    }
    Ok(0)
}
