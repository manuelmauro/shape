# shape

[![CI](https://github.com/manuelmauro/shape/actions/workflows/ci.yml/badge.svg)](https://github.com/manuelmauro/shape/actions/workflows/ci.yml)

**Shape Up for AI-agent-assisted software development.**

`shape` is a local-first, agent-agnostic CLI that wraps any AI coding agent (Claude Code, Cursor, aider, Windsurf, Copilot, Gemini, opencode, …) in the [Shape Up](https://basecamp.com/shapeup) loop: **shape → bet → build → ship**.

The human stays the judge. The AI co-pilots every phase — drafting pitches, hunting rabbit holes, proposing scope maps, and doing the building — but it cannot cross a phase boundary without a human **gate**. `shape` is the referee: it holds the artifacts as plain files in your repo, enforces the **appetite** as a hard resource budget, trips the **circuit breaker** when an agent runs long, and makes uncertainty visible on a **hill chart**.

> Shape Up exists to manage the risk of not shipping on time. That same apparatus maps almost 1:1 onto the failure modes of unsupervised agents — over-building, wandering into rabbit holes, losing the plot, and silently looping. See [`docs/PRODUCT.md`](docs/PRODUCT.md) for the full design and [`docs/shapeup/`](docs/shapeup/README.md) for the book.

> **Status:** v0.1 — the full Shape Up loop runs end to end (shape → bet → build → ship). `shape` was built by dogfooding itself: the [`.shape/`](.shape) workspace in this repo holds the pitches, bets, and scopes that drove its development.

## Installation

```bash
# Quick install (downloads a release binary, falls back to cargo)
curl -sSfL https://raw.githubusercontent.com/manuelmauro/shape/main/install.sh | sh

# Or from a clone
make install
```

## Quick start

```bash
shape init                       # scaffold .shape/ and .shaperc.toml in this repo
shape pitch new autopay          # co-shape a pitch (agent drafts, human edits)
shape derisk autopay             # rabbit-hole / risk pass
shape pitch ready autopay        # ▸ gate: mark shaped (validates 5 ingredients)
shape bet autopay --appetite small   # ▸ gate: place the bet, arm the circuit breaker
shape build                      # hand the bet to the agent; orchestrate the run
shape status                     # budget vs. appetite · scopes · hill · stuck dots
shape hill                       # render the hill chart
shape accept slice               # ▸ gate: accept the first integrated slice
shape ship                       # done = deployed; run the move-on checklist
```

Run `shape --help` and `shape <command> --help` for the full surface.

## Commands

| Command       | Phase  | Description                                                            |
| ------------- | ------ | --------------------------------------------------------------------- |
| `init`        | —      | Scaffold a `.shape/` workspace and `.shaperc.toml`                    |
| `pitch`       | shape  | Co-shape pitches (`new`, `list`, `show`, `ready`)                     |
| `derisk`      | shape  | Run the rabbit-hole / risk pass over a pitch                         |
| `bet`         | bet    | Place a bet on a shaped pitch and arm the circuit breaker (gate)     |
| `bets`        | bet    | List active and past bets                                            |
| `build`       | build  | Hand the active bet to the agent and orchestrate the run            |
| `scope`       | build  | Map and manage scopes (`list`, `add`, `split`, `done`)              |
| `hill`        | build  | Show the hill chart, or `hill set <scope> <0-100>`                  |
| `status`      | build  | Run dashboard: budget, scopes, hill, stuck dots                     |
| `stop`        | build  | Trip the circuit breaker manually                                   |
| `accept`      | build  | Accept a human gate (`slice` or `done`)                             |
| `ship`        | ship   | Mark done (deployed) and run the move-on checklist                  |
| `cooldown`    | —      | Enter or leave the between-runs cool-down                           |
| `completions` | —      | Generate shell completions                                          |

Global flags: `--dir <path>` (also `SHAPE_DIR`), `-q/--quiet`.

## Configuration

`--dir <path>` (and the `SHAPE_DIR` env var) point `shape` at a workspace directory and override everything else. With neither set, `shape` walks up from the working directory looking for `.shaperc.toml`; if found, its `dir` is used. With nothing configured, the default is `.shape`.

```toml
# .shaperc.toml — written by `shape init`
dir = ".shape"
```

A relative `dir` resolves against the location of the config file, so the same file works from any subdirectory.

## Workspace layout

`shape init` scaffolds a git-friendly, plain-text workspace:

```
.shape/
  pitch/   # shaped pitches: problem · appetite · solution · rabbit holes · no-gos
  bet/     # placed bets: a pitch committed to a run, with the armed budget
  scope/   # independently completable slices, with tasks and hill position
  hill/    # append-only hill-chart snapshots (how the work is moving)
```

## Development

```bash
make setup     # install rustfmt/clippy components, fetch deps, install git hooks
make ci        # fmt-check, clippy -D warnings, tests, build
make help      # list all targets
```

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) (enforced by a lefthook `commit-msg` hook).

## License

MIT OR Apache-2.0
