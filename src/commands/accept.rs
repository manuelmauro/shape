//! Accept a human gate: the first slice, or the finished work. (Stub — see
//! `docs/PRODUCT.md`.)

use crate::cli::{AcceptArgs, Cli};
use crate::error::Result;

/// Run the `accept` command.
pub fn run(args: &AcceptArgs, cli: &Cli) -> Result<i32> {
    super::unimplemented("accept", args, cli)
}
