# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project scaffold for the `shape` CLI: clap-based command surface mirroring the
  Shape Up loop (shape → bet → build → ship), `thiserror` error type, and
  `.shaperc.toml` discovery.
- `shape init` — scaffold a `.shape/` workspace (`pitches/`, `bets/`, `scopes/`,
  `hill/`) and a `.shaperc.toml`.
- `shape completions <shell>` — generate shell completions (bash, zsh, fish,
  PowerShell, elvish).
- The remaining commands (`pitch`, `derisk`, `bet`, `bets`, `build`, `scope`,
  `hill`, `status`, `stop`, `accept`, `ship`, `cooldown`) are present in the CLI
  surface and return a clear "not implemented yet" error pending build-out.
- Tooling: `Makefile`, `lefthook.yml` (pre-commit `make ci`, conventional-commit
  message check), GitHub Actions CI and tagged-release workflows, `install.sh`,
  pinned `rust-toolchain.toml`, and dual MIT/Apache-2.0 license.
- Product document at [`docs/PRODUCT.md`](docs/PRODUCT.md).
