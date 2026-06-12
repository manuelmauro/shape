//! Error types for the shape crate.

use thiserror::Error;

/// Errors that can occur during shape operations.
#[derive(Error, Debug)]
pub enum ShapeError {
    /// A shape workspace already exists at this location.
    #[error("shape workspace already initialized at {path} (a .shaperc.toml is present)")]
    AlreadyInitialized {
        /// Directory that already contains a shape workspace.
        path: String,
    },

    /// Parsing the configuration file failed.
    #[error("{path}: {message}")]
    Config {
        /// Config file path.
        path: String,
        /// Underlying error message.
        message: String,
    },

    /// A pitch name did not reduce to a usable slug.
    #[error("invalid pitch name '{0}': use letters, digits, and hyphens")]
    InvalidPitchName(String),

    /// A new pitch would overwrite an existing file.
    #[error("pitch '{name}' already exists at {path}")]
    PitchExists {
        /// Pitch name (slug).
        name: String,
        /// Existing file path.
        path: String,
    },

    /// A workspace document's frontmatter could not be parsed.
    #[error("{path}: {message}")]
    Parse {
        /// File path.
        path: String,
        /// Underlying parse error.
        message: String,
    },

    /// An artifact could not be found by name.
    #[error("{kind} not found: {name}")]
    NotFound {
        /// Kind of artifact (e.g. "pitch", "scope").
        kind: &'static str,
        /// Name that was looked up.
        name: String,
    },

    /// Tried to bet on a pitch that isn't shaped yet.
    #[error("pitch '{name}' is not shaped yet — run `shape pitch ready {name}` first")]
    PitchNotShaped {
        /// Pitch name (slug).
        name: String,
    },

    /// Tried to mark a pitch ready while it still has placeholders.
    #[error("pitch '{name}' still has TODO placeholders — fill in the five ingredients first")]
    PitchHasTodos {
        /// Pitch name (slug).
        name: String,
    },

    /// Tried to place a bet while one is already active.
    #[error("bet {id} is already active — ship or stop it before placing another")]
    ActiveBetExists {
        /// The active bet's id.
        id: String,
    },

    /// A command needs an active bet but none exists.
    #[error("no active bet — place one with `shape bet <pitch>`")]
    NoActiveBet,

    /// A new scope would overwrite an existing one.
    #[error("scope '{name}' already exists in this bet")]
    ScopeExists {
        /// Scope name (slug).
        name: String,
    },

    /// Tried to accept "done" while scopes remain unfinished.
    #[error("{count} scope(s) still todo — finish them before accepting done")]
    ScopesIncomplete {
        /// Number of scopes still in `todo`.
        count: usize,
    },

    /// A hill position outside the 0–100 range.
    #[error("invalid hill position {0}: must be between 0 and 100")]
    InvalidHillPosition(u8),

    /// A command is recognized and its arguments parse, but it has no behavior
    /// implemented yet. The CLI surface is defined in `docs/PRODUCT.md`.
    #[error("`shape {command}` is not implemented yet — see docs/PRODUCT.md")]
    NotImplemented {
        /// The command path that was invoked.
        command: String,
    },

    /// An I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// A specialized `Result` type for shape operations.
pub type Result<T> = std::result::Result<T, ShapeError>;
