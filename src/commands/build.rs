//! Hand the active bet to the agent and orchestrate the build run. (Stub — see
//! `docs/PRODUCT.md`.)

use crate::cli::{BuildArgs, Cli};
use crate::error::Result;

/// Run the `build` command.
pub fn run(args: &BuildArgs, cli: &Cli) -> Result<i32> {
    super::unimplemented("build", args, cli)
}
