---
name: pitch-lifecycle
title: Pitch Lifecycle
appetite: small
status: shaped
created: 2026-06-12
---

# Pitch Lifecycle

## Problem

`shape pitch new` writes a pitch, but there's no way to see what pitches exist,
read one, or move it from `shaping` to `shaped`. Without `ready` there's no
gate before betting — anyone (human or agent) could bet on a pitch that still
has `TODO:` placeholders, which is exactly the unshaped work the method warns
against.

## Appetite

**Small Batch** — one scope, a single agent session. These are three thin verbs
over files that already exist; no new state model is needed.

## Solution

Three subcommands reading the `pitches/*.md` frontmatter (`name`, `title`,
`appetite`, `status`):

- `pitch list` — one line per pitch: `STATUS  APPETITE  NAME  TITLE`. Headerless,
  pipe-friendly, arkouda-style.
- `pitch show <name>` — print the pitch file (resolve name → slug → path).
- `pitch ready <name>` — the gate. Refuse if any line still begins with the
  `TODO:` marker (the five ingredients aren't filled); otherwise flip
  `status: shaping → shaped` in place, preserving the body.

This needs a shared `workspace` module (locate the workspace, split/parse/
rewrite YAML frontmatter, list artifacts) that every later command reuses, plus
a `slug` module factored out of `pitch new`.

## Rabbit holes

- Frontmatter rewrite must preserve the body verbatim — split on the `\n---\n`
  fence, mutate only the parsed struct, recompose. Don't round-trip the body
  through a parser.
- "Filled in" is detected by no line *beginning* with a `TODO:` marker — cheap
  and good enough; no semantic validation. (Dogfooding caught the naive
  `contains` version: a pitch that mentions `TODO:` inline must still pass.)

## No-gos

- No editing of pitches through the CLI — humans/agents edit the Markdown.
- No `pitch rm` or rename; delete the file by hand if needed.
