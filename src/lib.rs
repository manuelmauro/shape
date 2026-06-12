//! `shape` — Shape Up for AI-agent-assisted software development.
//!
//! A local-first, agent-agnostic CLI that wraps any AI coding agent in the
//! Shape Up loop: shape → bet → build → ship. The human stays the judge; the
//! agent co-pilots each phase; this tool is the referee that holds the
//! artifacts, enforces the appetite as a budget, and makes uncertainty visible.
//!
//! See `docs/PRODUCT.md` for the full method and `docs/shapeup/` for the book.

pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod slug;
pub mod workspace;

pub use error::{Result, ShapeError};
