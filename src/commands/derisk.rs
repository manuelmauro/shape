//! Run the rabbit-hole / risk pass over a pitch. (Stub — see `docs/PRODUCT.md`.)

use crate::cli::{Cli, DeriskArgs};
use crate::error::Result;

/// Run the `derisk` command.
pub fn run(args: &DeriskArgs, cli: &Cli) -> Result<i32> {
    super::unimplemented("derisk", args, cli)
}
