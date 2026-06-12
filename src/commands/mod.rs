//! CLI command implementations.
//!
//! Each top-level command lives in its own module with a `run` entry point,
//! mirroring the `arkouda` / `skilo` layout. `init` and `completions` are
//! implemented; the rest of the surface is stubbed against
//! [`crate::error::ShapeError::NotImplemented`] until the build-out lands.

pub mod accept;
pub mod bet;
pub mod bets;
pub mod build;
pub mod completions;
pub mod cooldown;
pub mod derisk;
pub mod hill;
pub mod init;
pub mod pitch;
pub mod scope;
pub mod ship;
pub mod status;
pub mod stop;

use crate::cli::Cli;
use crate::error::{Result, ShapeError};
use colored::Colorize;

/// Stub for a command whose surface is defined but whose behavior is not built
/// yet. Echoes the parsed arguments (silent under `--quiet`) so the skeleton
/// demonstrates parsing, then returns
/// [`ShapeError::NotImplemented`](crate::error::ShapeError::NotImplemented) so
/// the exit code reflects reality.
pub(crate) fn unimplemented(command: &str, args: impl std::fmt::Debug, cli: &Cli) -> Result<i32> {
    if !cli.quiet {
        eprintln!("{} parsed `shape {command}`: {args:?}", "note:".dimmed());
    }
    Err(ShapeError::NotImplemented {
        command: command.to_string(),
    })
}

/// Like [`unimplemented`], for commands that take no arguments.
pub(crate) fn unimplemented_bare(command: &str, cli: &Cli) -> Result<i32> {
    if !cli.quiet {
        eprintln!(
            "{} `shape {command}` is not implemented yet",
            "note:".dimmed()
        );
    }
    Err(ShapeError::NotImplemented {
        command: command.to_string(),
    })
}
