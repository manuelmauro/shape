---
name: betting-and-building
title: Betting And Building
appetite: big
status: shaped
created: 2026-06-12
---

# Betting And Building

## Problem

After a pitch is `shaped` there's nowhere for it to go. There's no way to place
a bet (commit a pitch to a run with an armed budget), no notion of an "active
bet", and no way to break the work into scopes. `build`, `scope`, and
everything downstream (`hill`, `status`, `ship`) all need this run state to
exist first. This is the load-bearing architecture.

## Appetite

**Big Batch** — multiple scopes. It defines the core data model (`Bet`,
`Scope`, `Budget`) that the tracking/shipping pitch builds on, so it's worth a
full run.

## Solution

Establish the run state as files, then the verbs over it:

- **Bet** `bets/<NNN>-<slug>.md` — frontmatter `id`, `pitch`, `appetite`,
  `status` (active|building|shipped|stopped), `placed`, `budget {tokens,
  iterations, minutes}`, `slice_accepted`. The **active bet** is the one whose
  status is `active` or `building`.
- `shape bet <pitch> [--appetite] [--tokens/--iterations/--minutes]` — the gate.
  Requires the pitch to be `shaped`; refuses if an active bet already exists
  (keep the slate clean — one bet at a time). Default budgets come from the
  appetite. Writes the bet file in `active` status.
- `shape bets` — list bets (id, status, pitch, appetite).
- `shape build [--spike]` — require an active bet, flip it to `building`, and
  print a kickoff brief (pitch + current scopes). The CLI is the referee; it
  emits the brief, it doesn't run the agent.
- **Scope** `scopes/<bet-id>/<slug>.md` — frontmatter `name`, `bet`, `status`
  (todo|done), `hill` (0–100). Body is a free Markdown task checklist.
  - `scope add <name>` / `scope split <name>` — create a scope under the active
    bet (`split` is `add` framed as factoring work out).
  - `scope list` — scopes of the active bet with hill + status.
  - `scope done <name>` — flip `status: done`.

## Rabbit holes

- Default budgets per appetite are guesses; pick round numbers now (small:
  ~60k tokens / 40 iters / 20 min; big: ~200k / 120 / 90) and tune later.
- One active bet at a time keeps "active bet" unambiguous and avoids a
  selector flag on every downstream command.
- Bet id is `{count+1:03}` — simple, monotonic, no collisions for a solo loop.

## No-gos

- No parallel/multiple active bets in v1.
- No budget *enforcement* yet (no token metering) — `bet` only *arms* the
  numbers; the circuit breaker lands with tracking.
- No automatic scope discovery — scopes are added explicitly.
