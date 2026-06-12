//! Map and manage scopes — independently completable slices of work.

use crate::cli::{Cli, ScopeArgs, ScopeCommand, ScopeRefArgs};
use crate::error::{Result, ShapeError};
use crate::slug::{slugify, titleize};
use crate::workspace::{self, Scope};
use colored::Colorize;

/// Run the `scope` command.
pub fn run(args: &ScopeArgs, cli: &Cli) -> Result<i32> {
    match &args.command {
        ScopeCommand::List => list(cli),
        ScopeCommand::Add(ref_args) => add(ref_args, cli, false),
        ScopeCommand::Split(ref_args) => add(ref_args, cli, true),
        ScopeCommand::Done(ref_args) => done(ref_args, cli),
    }
}

/// One line per scope of the active bet: `HILL STATUS NAME`.
fn list(cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (_, bet) = workspace::require_active_bet(&ws)?;
    for (_, scope) in workspace::list_scopes(&ws, &bet.id)? {
        println!("{:>3}  {:5}{}", scope.hill, scope.status, scope.name);
    }
    Ok(0)
}

/// Create a scope under the active bet. `split` only changes the wording —
/// factoring a scope out is mechanically the same as adding one.
fn add(args: &ScopeRefArgs, cli: &Cli, split: bool) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (_, bet) = workspace::require_active_bet(&ws)?;

    let slug = slugify(&args.name);
    if slug.is_empty() {
        return Err(ShapeError::InvalidScopeName(args.name.clone()));
    }

    let dir = workspace::scopes_dir(&ws, &bet.id);
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{slug}.md"));
    if path.exists() {
        return Err(ShapeError::ScopeExists { name: slug });
    }

    let scope = Scope {
        name: slug.clone(),
        bet: bet.id.clone(),
        status: "todo".to_string(),
        hill: 0,
    };
    let body = format!(
        "
# {title}

Scope of bet {bet}.

## Tasks

- [ ] (add tasks; mark nice-to-haves with a leading `~`)
",
        title = titleize(&slug),
        bet = bet.id,
    );
    std::fs::write(&path, workspace::compose(&scope, &body))?;

    if !cli.quiet {
        let verb = if split { "Split out" } else { "Added" };
        println!(
            "{} {} scope '{}' in bet {}",
            "✓".green().bold(),
            verb,
            slug,
            bet.id
        );
    }
    Ok(0)
}

/// Mark a scope done (status `done`, hill 100).
fn done(args: &ScopeRefArgs, cli: &Cli) -> Result<i32> {
    let ws = workspace::resolve(cli)?;
    let (_, bet) = workspace::require_active_bet(&ws)?;
    let (path, scope) = workspace::find_scope(&ws, &bet.id, &args.name)?;

    workspace::rewrite::<Scope, _>(&path, |s| {
        s.status = "done".to_string();
        s.hill = 100;
    })?;

    if !cli.quiet {
        println!("{} Scope '{}' done.", "✓".green().bold(), scope.name);
    }
    Ok(0)
}
