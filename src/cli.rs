//! Command-line interface definitions.
//!
//! The surface mirrors the Shape Up loop documented in `docs/PRODUCT.md`:
//! shape → bet → build → ship, with a human gate between phases.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Main CLI application.
#[derive(Parser, Debug)]
#[command(name = "shape")]
#[command(author, version, about = "Shape Up for AI-agent-assisted software development", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Command,

    /// Path to the shape workspace directory. Overrides `dir` from
    /// `.shaperc.toml`. Defaults to `.shape`.
    #[arg(long, global = true, env = "SHAPE_DIR")]
    pub dir: Option<PathBuf>,

    /// Suppress non-essential informational output.
    #[arg(long, short, global = true)]
    pub quiet: bool,
}

/// Available CLI commands, grouped by phase of the Shape Up loop.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Scaffold a shape workspace (`.shape/`) and `.shaperc.toml` in this repo.
    Init,

    // ── Shape ───────────────────────────────────────────────
    /// Work with pitches — the shaped artifact (problem · appetite · solution ·
    /// rabbit holes · no-gos).
    Pitch(PitchArgs),

    /// Run the rabbit-hole / risk pass over a pitch (agent hunts holes; human
    /// approves patches).
    Derisk(DeriskArgs),

    // ── Bet ─────────────────────────────────────────────────
    /// Place a bet on a shaped pitch and arm the circuit breaker (human gate).
    Bet(BetArgs),

    /// List active and past bets.
    Bets,

    // ── Build ───────────────────────────────────────────────
    /// Hand the active bet to the agent and orchestrate the build run.
    Build(BuildArgs),

    /// Map and manage scopes — independently completable slices of work.
    Scope(ScopeArgs),

    /// Show or update the hill chart (uphill = figuring out, downhill = doing).
    Hill(HillArgs),

    /// Show the run dashboard: budget vs. appetite, scopes, hill, stuck dots.
    Status,

    /// Trip the circuit breaker manually and halt the active run.
    Stop,

    /// Accept a human gate: the first integrated slice, or the finished work.
    Accept(AcceptArgs),

    // ── Ship & cool-down ────────────────────────────────────
    /// Mark the work done (deployed) and run the move-on checklist.
    Ship,

    /// Enter or leave the between-runs cool-down period.
    Cooldown(CooldownArgs),

    /// Generate shell completions for the given shell.
    Completions(CompletionsArgs),
}

/// Appetite — the resource budget a raw idea is worth, set while shaping and
/// armed at the bet. Picks the default budget caps for the circuit breaker.
#[derive(ValueEnum, Clone, Copy, Debug)]
#[value(rename_all = "kebab-case")]
pub enum Appetite {
    /// Small Batch — one scope, a single agent session.
    Small,
    /// Big Batch — multiple scopes, possibly several sessions.
    Big,
}

impl Appetite {
    /// The value as written in pitch frontmatter (`small` / `big`).
    pub fn as_str(self) -> &'static str {
        match self {
            Appetite::Small => "small",
            Appetite::Big => "big",
        }
    }

    /// Human-facing label for the appetite.
    pub fn label(self) -> &'static str {
        match self {
            Appetite::Small => "Small Batch",
            Appetite::Big => "Big Batch",
        }
    }

    /// One-line description of what the appetite buys.
    pub fn summary(self) -> &'static str {
        match self {
            Appetite::Small => "one scope, a single agent session",
            Appetite::Big => "multiple scopes, possibly several sessions",
        }
    }
}

/// Arguments for the `pitch` command.
#[derive(Args, Debug)]
pub struct PitchArgs {
    /// What to do with pitches.
    #[command(subcommand)]
    pub command: PitchCommand,
}

/// Sub-commands of `pitch`.
#[derive(Subcommand, Debug)]
pub enum PitchCommand {
    /// Start co-shaping a new pitch (agent drafts, human edits).
    New(PitchNewArgs),
    /// List pitches and their state.
    List,
    /// Print a pitch.
    Show(PitchRefArgs),
    /// Mark a pitch shaped and ready to bet (validates the five ingredients).
    Ready(PitchRefArgs),
}

/// Arguments for `pitch new`.
#[derive(Args, Debug)]
pub struct PitchNewArgs {
    /// Short kebab-case name for the pitch.
    pub name: String,

    /// Appetite to shape against.
    #[arg(long, value_enum, default_value = "small")]
    pub appetite: Appetite,
}

/// Arguments that reference a pitch by name.
#[derive(Args, Debug)]
pub struct PitchRefArgs {
    /// Pitch name.
    pub name: String,
}

/// Arguments for the `derisk` command.
#[derive(Args, Debug)]
pub struct DeriskArgs {
    /// Pitch name to de-risk.
    pub name: String,
}

/// Arguments for the `bet` command.
#[derive(Args, Debug)]
pub struct BetArgs {
    /// Pitch name to bet on.
    pub name: String,

    /// Appetite (sets the default budget caps).
    #[arg(long, value_enum, default_value = "small")]
    pub appetite: Appetite,

    /// Override the token ceiling for the circuit breaker.
    #[arg(long)]
    pub tokens: Option<u64>,

    /// Override the iteration ceiling for the circuit breaker.
    #[arg(long)]
    pub iterations: Option<u32>,

    /// Override the wall-clock ceiling (minutes) for the circuit breaker.
    #[arg(long)]
    pub minutes: Option<u32>,
}

/// Arguments for the `build` command.
#[derive(Args, Debug)]
pub struct BuildArgs {
    /// R&D spike mode: looser shaping, no ship gate, output is
    /// throwaway-by-default.
    #[arg(long)]
    pub spike: bool,
}

/// Arguments for the `scope` command.
#[derive(Args, Debug)]
pub struct ScopeArgs {
    /// What to do with scopes.
    #[command(subcommand)]
    pub command: ScopeCommand,
}

/// Sub-commands of `scope`.
#[derive(Subcommand, Debug)]
pub enum ScopeCommand {
    /// List scopes for the active bet.
    List,
    /// Add a new scope.
    Add(ScopeRefArgs),
    /// Split a scope into smaller scopes.
    Split(ScopeRefArgs),
    /// Mark a scope done.
    Done(ScopeRefArgs),
}

/// Arguments that reference a scope by name.
#[derive(Args, Debug)]
pub struct ScopeRefArgs {
    /// Scope name.
    pub name: String,
}

/// Arguments for the `hill` command.
#[derive(Args, Debug)]
pub struct HillArgs {
    /// Update a position; omit to render the chart.
    #[command(subcommand)]
    pub command: Option<HillCommand>,
}

/// Sub-commands of `hill`.
#[derive(Subcommand, Debug)]
pub enum HillCommand {
    /// Set a scope's position on the hill.
    Set(HillSetArgs),
}

/// Arguments for `hill set`.
#[derive(Args, Debug)]
pub struct HillSetArgs {
    /// Scope name.
    pub scope: String,

    /// Position from 0 (start, uphill) to 100 (done, downhill).
    pub position: u8,
}

/// Arguments for the `accept` command.
#[derive(Args, Debug)]
pub struct AcceptArgs {
    /// Which human gate to accept.
    #[arg(value_enum)]
    pub gate: Gate,
}

/// A human approval gate in the build phase.
#[derive(ValueEnum, Clone, Copy, Debug)]
#[value(rename_all = "kebab-case")]
pub enum Gate {
    /// The first integrated vertical slice (the demo gate).
    Slice,
    /// The finished, integrated, tested work.
    Done,
}

/// Arguments for the `cooldown` command.
#[derive(Args, Debug)]
pub struct CooldownArgs {
    /// Start or end cool-down; omit to show cool-down status.
    #[arg(value_enum)]
    pub action: Option<CooldownAction>,
}

/// Whether to begin or end cool-down.
#[derive(ValueEnum, Clone, Copy, Debug)]
#[value(rename_all = "kebab-case")]
pub enum CooldownAction {
    /// Begin cool-down.
    Start,
    /// End cool-down.
    End,
}

/// Arguments for the `completions` command.
#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for.
    #[arg(value_enum)]
    pub shell: Shell,
}

/// Shells supported by `shape completions`.
#[derive(ValueEnum, Clone, Copy, Debug)]
#[value(rename_all = "kebab-case")]
pub enum Shell {
    /// GNU Bash.
    Bash,
    /// Z shell.
    Zsh,
    /// Fish shell.
    Fish,
    /// PowerShell.
    PowerShell,
    /// Elvish shell.
    Elvish,
}
