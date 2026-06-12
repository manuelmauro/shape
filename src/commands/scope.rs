//! Map and manage scopes — independently completable slices of work. (Stub —
//! see `docs/PRODUCT.md`.)

use crate::cli::{Cli, ScopeArgs};
use crate::error::Result;

/// Run the `scope` command.
pub fn run(args: &ScopeArgs, cli: &Cli) -> Result<i32> {
    super::unimplemented("scope", args, cli)
}
