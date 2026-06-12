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
