---
name: tracking-and-shipping
title: Tracking And Shipping
appetite: big
status: shaped
created: 2026-06-12
---

# Tracking And Shipping

## Problem

Once a bet is building with scopes, there's no way to see how the work is
moving, no way to halt a run, and no way to finish one. The hill chart — the
method's key signal for "what's stuck" — doesn't exist, and the human gates
(`accept`, `ship`) and the cool-down period have no commands. This pitch closes
the loop.

## Appetite

**Big Batch** — it spans the whole back half of the loop (tracking, the gates,
shipping, cool-down) over the run state from the betting pitch.

## Solution

Built on the active bet and its scopes:

- `shape hill set <scope> <0-100>` — update the scope's `hill` field and append
  a snapshot line to `hill/<bet-id>.jsonl` (`{time, scope, position}`).
- `shape hill` — render the active bet's scopes as an ASCII hill (uphill =
  figuring out, downhill = doing); a dot stuck uphill is the raised hand.
- `shape status` — the dashboard: active bet, appetite/budget, a scopes table
  (name, hill, status), and counts. The first stop when anxious about a run.
- `shape stop` — trip the circuit breaker: flip the active bet to `stopped`.
- `shape accept slice|done` — the human gates. `slice` sets `slice_accepted`;
  `done` is allowed only when every scope is `done`.
- `shape ship` — flip the bet to `shipped` and print the move-on reminder
  (new requests are raw ideas for the next round).
- `shape cooldown start|end` — toggle a `cooldown` marker file in the workspace;
  no args prints whether cool-down is active.
- `shape derisk <name>` — print a pitch's Rabbit holes and No-gos sections for a
  focused risk review (a read action, not a state change).

## Rabbit holes

- Hill snapshots are timestamped with the local clock; the `.jsonl` is
  append-only so history (how work *moves*) survives.
- "Stuck" detection is shallow without agent telemetry — `status` shows
  positions and flags scopes still uphill; it doesn't infer looping.
- `accept done` should refuse if any scope is still `todo` — the gate has teeth.

## No-gos

- No real-time budget metering / automatic circuit-breaker trip in v1 — `stop`
  is manual.
- No multi-bet status aggregation (one active bet).
- No TUI; `hill` and `status` are plain ASCII for the terminal.
