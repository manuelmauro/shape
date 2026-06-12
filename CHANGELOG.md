# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project scaffold for the `shape` CLI: clap-based command surface mirroring the
  Shape Up loop (shape → bet → build → ship), `thiserror` error type, and
  `.shaperc.toml` discovery.
- `shape init` — scaffold a `.shape/` workspace (`pitch/`, `bet/`, `scope/`,
  `hill/`) and a `.shaperc.toml`.
- `shape pitch new <name> [--appetite small|big]` — scaffold a pitch from the
  five-ingredient template (problem · appetite · solution · rabbit holes ·
  no-gos), normalizing the name into a slug and refusing to clobber an existing
  pitch.
- `shape pitch list`/`show`/`ready` — list pitches, print one, and the shaped
  gate (refuses while any line begins with the `TODO:` marker).
- `shape bet`/`bets` — place a bet on a shaped pitch (arming a token/iteration/
  minute budget; refuses a second active bet) and list bets.
- `shape build [--spike]` — flip the active bet to building and print a kickoff
  brief (the pitch's Problem and Solution, plus current scopes).
- `shape scope add`/`split`/`list`/`done` — map and complete scopes.
- `shape hill`/`hill set <scope> <0-100>` — render the ASCII hill chart and
  update a scope's position, with an append-only snapshot log.
- `shape status` — the run dashboard; `shape stop` — trip the circuit breaker.
- `shape accept slice`/`done` (the human gates; `done` refuses unfinished
  scopes), `shape ship`, and `shape cooldown start`/`end`.
- `shape derisk <pitch>` — print a pitch's rabbit holes and no-gos.
- `shape completions <shell>` — generate shell completions (bash, zsh, fish,
  PowerShell, elvish).
- `skills/use-shape/` — a portable, skilo-validated agent skill teaching agents
  the Shape Up loop, gate discipline, and the file-based artifact contract.
- `shape` was developed by dogfooding itself; the `.shape/` workspace in this
  repo records the pitches, bets, and scopes that drove the build.
- Tooling: `Makefile`, `lefthook.yml` (pre-commit `make ci`, conventional-commit
  message check), GitHub Actions CI and tagged-release workflows, `install.sh`,
  pinned `rust-toolchain.toml`, and dual MIT/Apache-2.0 license.
- Product document at [`docs/PRODUCT.md`](docs/PRODUCT.md).
