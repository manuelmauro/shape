---
name: use-shape
description: Run the Shape Up loop (shape → bet → build → ship) for AI-agent-assisted development with the `shape` CLI. Invoke when working in a repo that has a `.shape/` workspace or a `.shaperc.toml`; when the user talks about shaping work, pitches, appetites, bets, scopes, or hill charts; or when you're about to build a non-trivial feature and need to bound scope, avoid over-building, and respect human approval gates.
license: MIT OR Apache-2.0
---

# Using shape

`shape` brings the [Shape Up](https://basecamp.com/shapeup) method to AI-agent-assisted development. It runs the build loop in four phases — **shape → bet → build → ship** — with a **human approval gate** between each one. You (the agent) co-pilot every phase: you draft pitches, hunt rabbit holes, propose scope maps, and do the building. The human is the judge; `shape` is the referee that holds the artifacts as plain files, enforces the appetite as a budget, and tracks progress on a hill chart.

The single most important rule: **do the work inside a phase; stop at the gate.** See "The loop and the gates" below.

> `shape` is young. `shape init` and `shape completions` work today; the other commands are defined but may print "not implemented yet". The methodology, the gate discipline, and the file-based workspace below are the durable contract. Always check `shape --help` for what is actually wired up, and when a command isn't ready, do the equivalent by editing the `.shape/` files directly.

## When to use

- You're in a repo with a `.shape/` workspace or a `.shaperc.toml`.
- The user talks about shaping, pitches, appetite, bets, scopes, hill charts, or "the cycle".
- You're about to build a non-trivial feature and want to bound the scope before diving in.
- You catch yourself about to over-build, gold-plate, or refactor beyond what was asked — that's the cue to shape and set an appetite instead.

## The loop and the gates

An idea moves through four phases. A `▸` marks a **human gate** you must not cross yourself:

1. **Shape** — co-shape a pitch (problem · appetite · solution · rabbit holes · no-gos). ▸ the human marks it *ready*.
2. **Bet** — the human picks a pitch and arms the budget. ▸ placing a bet is a human decision.
3. **Build** — you build inside the bet: get one slice done, map scopes, keep the hill honest. ▸ the human accepts the first slice, then the finished work.
4. **Ship** — done = deployed; new requests become raw ideas for the next round.

**Gate discipline — the heart of this skill:**

- Draft, propose, prepare — then hand off. Say *"this pitch is ready for your review"* rather than marking it ready.
- Never place a bet, accept a slice, accept "done", or ship on the human's behalf.
- A blown budget (the **circuit breaker**) means **stop and re-shape**, not push harder. Only continue past budget if every scope is already downhill (no unknowns left) and the human says so.

## Philosophy

- **Appetite, not estimate.** The human sets a budget (tokens · iterations · time). *Fixed budget, variable scope* — you cut scope to fit the budget, never grow the budget to fit the scope.
- **Scope hammering.** Constantly separate must-haves from nice-to-haves. Mark nice-to-haves with a leading `~`; default to cutting them. Compare **down to baseline** (what users do today), not up to an ideal.
- **Get one piece done.** Build one vertical slice first — **core, small, novel** — and make it demoable before fanning out. Don't build horizontally (every layer stubbed, nothing actually working).
- **Make uncertainty visible.** Report each scope's hill position: *uphill* = still figuring out the approach; *downhill* = just executing. Don't claim downhill until you've validated the approach by building, not just thinking. A dot that won't move is a signal to ask for help or re-shape — agents loop silently, so say so.
- **Leave room.** A shaped pitch is rough on purpose. It's boundaries and rules, not a spec. Fill in the real details as you build.

## Where the workspace lives

Don't hardcode `.shape/`. Resolution order:

1. `--dir <path>` flag (one-shot override).
2. `SHAPE_DIR=<path>` environment variable (session override).
3. `dir = "..."` in `.shaperc.toml`, discovered by walking up from the cwd. A relative path resolves against the config file's directory.
4. Default: `.shape/`.

If there's no workspace yet and the `shape` binary is installed, run `shape init` to scaffold one.

## Commands

Run `shape --help` and `shape <command> --help` for the authoritative, currently-wired surface. The intended surface:

- `shape init` — scaffold `.shape/` (`pitches/`, `bets/`, `scopes/`, `hill/`) and `.shaperc.toml`. *(works today)*
- `shape completions <shell>` — shell completions for bash, zsh, fish, PowerShell, elvish. *(works today)*
- `shape pitch new|list|show|ready <name>` — co-shape and manage pitches.
- `shape derisk <name>` — rabbit-hole / risk pass over a pitch.
- `shape bet <name> [--appetite small|big]` — place a bet and arm the circuit breaker (human gate).
- `shape bets` — list active and past bets.
- `shape build [--spike]` — hand the active bet to the agent and orchestrate the run.
- `shape scope list|add|split|done` — map and manage scopes.
- `shape hill [set <scope> <0-100>]` — show or update the hill chart.
- `shape status` — run dashboard: budget vs. appetite, scopes, hill, stuck dots.
- `shape stop` — trip the circuit breaker manually.
- `shape accept slice|done` — accept a human gate.
- `shape ship` — mark done (deployed) and run the move-on checklist.
- `shape cooldown [start|end]` — enter or leave the between-runs period.

Global flags: `--dir <path>` (also `SHAPE_DIR`), `-q/--quiet`.

When a command reports "not implemented yet", do the equivalent by editing the `.shape/` files (next section) and tell the human exactly what you changed.

## The artifact contract

The workspace is plain text under the resolved `dir` (default `.shape/`), so it's diffable and git-friendly:

- `pitches/<name>.md` — a shaped pitch with five ingredients: **Problem** (a concrete story showing why the status quo fails — the baseline), **Appetite** (small or big, and why), **Solution** (rough — breadboards: a `Place`, affordances under it, `→` connections; or low-fidelity sketches), **Rabbit holes** (called-out risks and their patches), **No-gos** (explicitly excluded to fit the appetite). Rough and bounded, never a spec.
- `bets/<id>-<name>.md` — a placed bet: the pitch committed to a run, with the armed budget. Human-owned.
- `scopes/<bet>/<scope>.md` — an independently completable slice: a checklist of tasks (must-haves; `~` for nice-to-haves) plus the scope's hill position.
- `hill/<bet>.jsonl` — append-only hill snapshots, so the human can see how the work is *moving*, not just where it is.

## Workflows

**Shape a pitch**

1. Narrow the problem: ask "when did this actually bite?" to find the real use case, not the feature label.
2. Propose an appetite (small/big) and design only what fits inside it.
3. Sketch the solution roughly — breadboard the flow (places, affordances, connections) rather than drawing pixels.
4. Attack your own concept: list rabbit holes and patch or fence each one; mark the no-gos.
5. Write `pitches/<name>.md` with all five ingredients, then say it's ready for review — don't self-mark it ready.

**Build inside a bet**

1. Get oriented, then pick the first slice — core, small, novel — and make it work end-to-end.
2. Discover tasks by doing the work; map them into scopes as the real structure emerges (not up front).
3. Keep the hill honest and surface anything stuck early.
4. Scope-hammer continuously; mark nice-to-haves `~` and default to cutting them.
5. Stop at the slice gate and the done gate for the human to accept.

## What not to do

- Don't cross a human gate (ready / bet / accept / ship) on your own — draft and hand off.
- Don't over-build or gold-plate; cut scope to the appetite and compare down to baseline.
- Don't build horizontally — land one working vertical slice before spreading out.
- Don't claim a scope is downhill until you've validated the approach by building it.
- Don't push past a blown budget; stop and re-shape.
- Don't hardcode `.shape/` in scripts — resolve via `--dir` / `SHAPE_DIR` / `.shaperc.toml` / default.
- Don't treat the pitch as a spec; it's rough boundaries you fill in while building.
- Don't invent new artifact shapes — follow the contract above, or run `shape init` first.
