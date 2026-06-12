//! Enter or leave the between-runs cool-down period. (Stub — see
//! `docs/PRODUCT.md`.)

use crate::cli::{Cli, CooldownArgs};
use crate::error::Result;

/// Run the `cooldown` command.
pub fn run(args: &CooldownArgs, cli: &Cli) -> Result<i32> {
    super::unimplemented("cooldown", args, cli)
}
