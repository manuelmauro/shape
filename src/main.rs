//! The `shape` CLI binary: parse arguments and dispatch to a command.

use clap::Parser;
use colored::Colorize;
use shape::cli::{Cli, Command};
use shape::{Result, commands};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(0) => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("{} {error}", "error:".red().bold());
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> Result<i32> {
    match &cli.command {
        Command::Init => commands::init::run(cli),
        Command::Pitch(args) => commands::pitch::run(args, cli),
        Command::Derisk(args) => commands::derisk::run(args, cli),
        Command::Bet(args) => commands::bet::run(args, cli),
        Command::Bets => commands::bets::run(cli),
        Command::Build(args) => commands::build::run(args, cli),
        Command::Scope(args) => commands::scope::run(args, cli),
        Command::Hill(args) => commands::hill::run(args, cli),
        Command::Status => commands::status::run(cli),
        Command::Stop => commands::stop::run(cli),
        Command::Accept(args) => commands::accept::run(args, cli),
        Command::Ship => commands::ship::run(cli),
        Command::Cooldown(args) => commands::cooldown::run(args, cli),
        Command::Completions(args) => commands::completions::run(args),
    }
}
