//! Place a bet on a shaped pitch and arm the circuit breaker. (Stub — see
//! `docs/PRODUCT.md`.)

use crate::cli::{BetArgs, Cli};
use crate::error::Result;

/// Run the `bet` command.
pub fn run(args: &BetArgs, cli: &Cli) -> Result<i32> {
    super::unimplemented("bet", args, cli)
}
