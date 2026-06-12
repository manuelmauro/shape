//! Trip the circuit breaker manually and halt the active run. (Stub — see
//! `docs/PRODUCT.md`.)

use crate::cli::Cli;
use crate::error::Result;

/// Run the `stop` command.
pub fn run(cli: &Cli) -> Result<i32> {
    super::unimplemented_bare("stop", cli)
}
