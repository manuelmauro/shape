//! Hand the active bet to the agent and orchestrate the build run.
//!
//! The CLI is the referee: it flips the bet to `building` and emits a kickoff
//! brief (the pitch's problem and solution, plus the current scopes). It does
//! not run the agent.

use crate::cli::{BuildArgs, Cli};
use crate::error::Result;
use crate::workspace::{self, Bet};
use colored::Colorize;

/// Run the `build` command.
pub fn run(args: &BuildArgs, cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (bet_path, bet) = workspace::require_active_bet(&ws)?;

    if bet.status == "active" {
        workspace::rewrite::<Bet, _>(&bet_path, |b| b.status = "building".to_string())?;
    }

    let (pitch_path, pitch) = workspace::find_pitch(&ws, &bet.pitch)?;
    let text = std::fs::read_to_string(&pitch_path)?;
    let body = workspace::split_frontmatter(&text)
        .map(|(_, b)| b)
        .unwrap_or(text.as_str());

    println!(
        "{}",
        format!("Kickoff — bet {} · {}", bet.id, pitch.title).bold()
    );
    println!(
        "Appetite: {} · budget {} tokens · {} iterations · {} min",
        bet.appetite, bet.budget.tokens, bet.budget.iterations, bet.budget.minutes
    );
    if args.spike {
        println!(
            "Mode: {} — no ship gate; output is throwaway-by-default",
            "spike".yellow()
        );
    }

    if let Some(problem) = workspace::section(body, "Problem") {
        println!("\n## Problem\n\n{problem}");
    }
    if let Some(solution) = workspace::section(body, "Solution") {
        println!("\n## Solution\n\n{solution}");
    }

    println!("\n## Scopes");
    let scopes = workspace::list_scopes(&ws, &bet.id)?;
    if scopes.is_empty() {
        println!(
            "(none yet — get one piece done: `shape scope add <name>` for a core, small, novel slice)"
        );
    } else {
        for (_, scope) in scopes {
            println!("- {} [{}] hill {}", scope.name, scope.status, scope.hill);
        }
    }
    Ok(0)
}
