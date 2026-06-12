# `shape` — Product Document

> **Shape Up for AI-agent-assisted software development.**
> A local-first, agent-agnostic CLI that brings the discipline of shaping, betting, and bounded building to a workflow where AI agents do most of the typing.

**Status:** Draft v0.1 · 2026-06-12
**Audience:** contributors to `shape`, early adopters, and anyone evaluating the methodology
**Companion reading:** [`docs/shapeup/`](shapeup/README.md) — the full Shape Up book by Ryan Singer (Basecamp), which this product reinterprets.

---

## 1. Summary

`shape` is a command-line tool that wraps any AI coding agent (Claude Code, Cursor, aider, Windsurf, Copilot, Gemini, opencode, …) in the **Shape Up** product-development loop: **shape → bet → build → ship**.

The human stays the judge. The AI co-pilots every phase — drafting pitches, hunting rabbit holes, proposing scope maps, and doing the building — but it cannot cross a phase boundary without a human **gate**. `shape` is the referee: it holds the artifacts (as plain files in your repo), enforces the **appetite** as a hard resource budget, trips the **circuit breaker** when an agent runs long, and makes uncertainty visible on a **hill chart**.

It is **local-first** (files + git, no SaaS), **agent-agnostic** (adapters teach any agent the workflow), and **scale-adaptive** (a solo builder can wing it; a team can turn on the full ceremony).

---

## 2. The problem

AI coding agents are fast and tireless, which means their failure modes are fast and tireless too. Anyone who has handed a real task to an autonomous agent has watched at least one of these:

- **Over-building.** The agent gold-plates. Asked for a one-day fix, it refactors three modules, adds a config system, and writes a plugin architecture nobody requested. There is no internal sense of "enough."
- **Wandering into rabbit holes.** It hits an unforeseen hard problem and grinds — burning tokens and time on a sub-problem that a human would have designed around up front.
- **Losing the plot.** Long tasks overflow the context window. The agent forgets the original goal, contradicts earlier decisions, or re-solves something it already solved.
- **Building horizontally.** It stubs every layer and integrates none, so at the end "lots of things are done but nothing works."
- **Silent looping & overconfidence.** Worst of all, agents rarely raise their hand. They won't say _"I don't know how to solve this."_ They loop quietly or report confident, hallucinated progress.
- **No supervisory visibility.** The human can't tell whether a half-finished agent run is on track, stuck, or off in the weeds — until the bill arrives or the diff is unreviewable.

The reflex is to manage this with ever-more-detailed prompts and task lists — the agent equivalent of over-specifying wireframes. That makes things worse: it removes the agent's room to find the real implementation, and it still doesn't bound the cost.

## 3. The insight

Shape Up was written to manage **one specific risk for human teams: the risk of not shipping on time.** Its entire apparatus exists to keep work bounded, de-risked, integrated early, and honestly tracked.

That apparatus maps almost 1:1 onto the failure modes above. The time dimension collapses — agents work in minutes and hours, not six-week calendars — but the _structural_ risks are the same shape, and Shape Up's tools are uncannily well-suited to govern an agent:

| Agent failure mode                     | Shape Up tool that addresses it                                                                  |
| -------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Over-building / gold-plating           | **Appetite** (fixed budget, variable scope), **no-gos**, **scope hammering** (`~` nice-to-haves) |
| Wandering into rabbit holes            | **Shaping** removes unknowns up front; **circuit breaker** caps the downside                     |
| Losing the plot across context windows | **Scopes** sized to one integrable, context-window-fitting slice                                 |
| Building horizontally                  | **Get one piece done** — vertical integration first (core / small / novel)                       |
| Silent looping & overconfidence        | **Hill chart** — a dot that doesn't move is a raised hand the agent won't raise itself           |
| No supervisory visibility              | **Hill chart history** + **status**: see what's in motion, what's stuck                          |

So `shape` isn't "Shape Up, but with AI bolted on." It's the observation that **shaping and bounding are exactly the controls unsupervised agents lack** — and that a human + a fleet of agents already resembles the manager + build-team structure the book was written for. The solo developer of 2026 commands a team. `shape` gives them the playbook for running it.

---

## 4. What `shape` is (and is not)

**`shape` is:**

- A **process harness** that structures the shape → bet → build → ship loop as commands and files.
- A **referee** that enforces gates (human approval between phases) and budgets (the circuit breaker).
- An **artifact store**: pitches, bets, scopes, and hill snapshots live as version-controlled files in your repo.
- An **agent teacher**: it installs workflow instructions/skills into whatever agent you use, so the agent knows how to read and update the artifacts.

**`shape` is not:**

- **Not an agent or a model.** It orchestrates the agent you already use; it doesn't generate code itself.
- **Not a replacement for your coding agent.** Claude Code / Cursor / aider still do the work.
- **Not a SaaS or a ticketing system.** No backlog, no server, no account. Files and git.
- **Not a CI/CD or deploy tool.** "Done means deployed," but `shape` calls your deploy; it doesn't own it.

---

## 5. Core concepts: Shape Up → `shape`

The vocabulary is preserved deliberately — it's good, and it gives humans and agents a shared language. What changes is the _unit of measure_ and _who does the work_.

| Shape Up (human teams)             | `shape` (human + agents)                                                                                                       |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Six-week cycle**                 | A **run**: one bounded build session against a placed bet. Bounded by budget, not calendar.                                    |
| **Appetite** (time budget)         | **Appetite as a resource budget** — tokens · iterations · wall-clock. Declared while shaping, armed at the bet.                |
| **Fixed time, variable scope**     | **Fixed budget, variable scope** — the agent hammers scope to fit the budget, never the reverse.                               |
| **Shaping** (closed-door, senior)  | **Co-shaping** — the agent drafts; the human edits and approves. Still deliberately rough.                                     |
| **Breadboard / fat-marker sketch** | Text artifacts: breadboard notation (places · affordances · connections) and markdown/ASCII low-fi sketches in the pitch file. |
| **Pitch**                          | A `pitch` markdown file: problem · appetite · solution · rabbit holes · no-gos.                                                |
| **Betting table**                  | The **bet gate**. Solo: one `shape bet` command. Team: a multi-stakeholder review.                                             |
| **No backlog**                     | Un-bet pitches are just files. Nothing to groom. Important ideas come back.                                                    |
| **Circuit breaker**                | **Hard budget cap.** Exceed the appetite → `shape` halts the run by default. No automatic extension.                           |
| **Cool-down**                      | **Cool-down** — between runs: agent-assisted bug fixing, retros, spikes, and re-shaping.                                       |
| **Hand over responsibility**       | The agent receives the **whole pitch**, not a task list. It discovers its own tasks.                                           |
| **Get one piece done**             | The agent must land one **vertical slice** (core · small · novel) and pass a demo gate before fanning out.                     |
| **Scopes**                         | Independently completable slices, each sized to fit an agent's working context. Tracked as files.                              |
| **Hill chart**                     | Per-scope **uphill → downhill** status, self-reported by the agent and snapshotted over time.                                  |
| **Imagined vs. discovered tasks**  | The agent's task list **grows** as it works. `shape` expects and shows this.                                                   |
| **Scope hammering**                | The human (and the agent, when prompted) cuts `~` nice-to-haves to fit appetite — the antidote to gold-plating.                |
| **Compare to baseline**            | Stop condition: judge agent output _down to baseline_, not _up to ideal_.                                                      |
| **QA is for the edges**            | The agent owns basic quality (writes its tests); a QA pass (agent or human) hunts edges near the end.                          |
| **Move on / stay debt-free**       | Post-run feedback is _raw ideas_ → re-shape; don't interrupt the run.                                                          |

---

## 6. The `shape` loop

Four phases. The AI co-pilots inside each; a **human gate** (`▸`) separates them. The agent cannot advance the artifact's state past a gate on its own — that's the core safety property.

```
 RAW IDEA
    │
    ▼
┌─────────────┐         ┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│  1. SHAPE   │  ▸gate  │   2. BET    │  ▸gate  │  3. BUILD   │  ▸gate  │  4. SHIP    │
│ co-shaping  │ ──────► │ commit +    │ ──────► │ agent builds│ ──────► │ done = de-  │
│ (AI drafts, │ "shaped"│ arm budget  │ "bet"   │ in guardrail│ "slice" │ ployed;     │
│  human OKs) │         │ (human)     │         │ (human OKs) │ + "done"│ move on     │
└─────────────┘         └─────────────┘         └─────────────┘         └─────────────┘
                                                        │  ▲
                                                  circuit breaker
                                                  (budget exceeded → halt)
```

### Phase 1 — Shape (co-shaping)

The agent does the legwork; the human exercises judgment.

- **Set boundaries.** The agent helps narrow the raw idea — running the book's _"when did you want it?"_ move to find the real use case — and proposes an **appetite** (Small Batch / Big Batch). The human sets it.
- **Find the elements.** The agent produces a **breadboard** (places, affordances, connection lines) and/or fat-marker-equivalent sketches at deliberately low fidelity — text and markdown, not pixel-perfect mocks.
- **Risks & rabbit holes.** The agent is explicitly prompted to _attack its own concept_: walk the use case in slow motion, surface technical unknowns, flag holes, and propose patches or **no-gos**. (This is where AI leverage is highest — tireless adversarial review of the plan before any code is written.)
- **Write the pitch.** The agent assembles the five ingredients into a `pitch` file. `shape` validates all five are present.

> **▸ Gate "shaped":** the human reviews and marks the pitch ready. A pitch missing a problem, an appetite, or a solution cannot pass — that's "unshaped work," and handing it to an agent is how rabbit holes happen.

### Phase 2 — Bet (the gate)

- The agent can compare candidate pitches, surface the riskiest assumption in each, and estimate budget fit — but **does not decide.**
- The human **places the bet**: picks the pitch and confirms the appetite. This **arms the circuit breaker** (the budget cap) for the run.
- Solo builder: one command. Team: a real betting-table review (see §10).

> **▸ Gate "bet":** budget armed, run created, agent cleared to build.

### Phase 3 — Build (agent builds, human supervises)

- **Hand over the whole pitch, not tasks.** The agent gets oriented, discovers its own tasks, and proposes a **scope map** (the human can redraw the lines).
- **Get one piece done.** Before fanning out, the agent must land one **vertical slice** — chosen for being **core, small, and novel** — and make it demoable. This counters the "start with the boilerplate" reflex and proves the risky idea early.
- **Show progress.** The agent self-reports each scope's **hill position** (uphill = still figuring out the approach; downhill = just executing). `shape` snapshots these over time.
- **`shape` enforces.** It meters the budget and trips the **circuit breaker** if the run runs long. It watches for **stuck dots** — a hill position that hasn't moved while budget burns is the raised hand an agent won't raise — and surfaces them to the human.
- **Scope hammering.** The human (and the agent, prompted) continuously cut `~` nice-to-haves to fit the appetite.

> **▸ Gate "slice":** human accepts the first integrated slice (demo).
> **▸ Gate "done":** human accepts the finished, integrated, tested work.

### Phase 4 — Ship & move on

- "Done means deployed." `shape` triggers your deploy path (it doesn't own it).
- The agent drafts the changelog / announcement and lists discovered debt.
- New requests that arrive are treated as **raw ideas** — gently declined for _this_ run, fed back to Phase 1. **Stay debt-free; keep the slate clean.**
- Optionally enter **cool-down**: agent-assisted bug fixing, spikes, and re-shaping the next pitches.

---

## 7. CLI surface

Commands mirror the loop. Everything is also expressible as file edits, so an agent can drive `shape` either by calling it or by editing artifacts.

```
shape init                      # scaffold .shape/ and install agent adapter(s)

# ── Shape ───────────────────────────────────────────────
shape pitch new <name>          # start co-shaping; agent drafts, human edits
shape pitch list                # list pitches and their state (raw/shaping/shaped)
shape pitch show <name>
shape derisk <name>             # run the rabbit-hole / risk pass on a pitch
shape pitch ready <name>        # ▸ gate: mark shaped (validates 5 ingredients)

# ── Bet ─────────────────────────────────────────────────
shape bet <name> [--appetite small|big] [--budget tokens=…,iters=…,time=…]
                                # ▸ gate: place the bet, arm the circuit breaker
shape bets                      # show active and past bets

# ── Build ───────────────────────────────────────────────
shape build                     # hand the active bet to the agent; orchestrate the run
shape scope list|add|split|done # map and manage scopes (often agent-proposed)
shape hill                      # render the hill chart in the terminal
shape hill set <scope> <0-100>  # update a scope's position (agent or human)
shape status                    # budget spent vs. appetite · scopes · hill · stuck dots
shape stop                      # manual circuit breaker — halt the run
shape accept slice|done         # ▸ gates: accept the first slice / finished work

# ── Ship & cool-down ────────────────────────────────────
shape ship                      # mark done = deployed; run the move-on checklist
shape cooldown [start|end]      # enter/leave the between-runs period
```

---

## 8. Artifacts & file model

Local-first and git-native: every artifact is a diffable text file the agent can read and write. No database, no lock-in.

```
.shape/
  config.toml              # appetite presets, default budgets, agent adapter(s)
  pitch/
    autopay.md             # frontmatter: state, appetite · body: 5 ingredients + breadboard
  bet/
    001-autopay.md         # the placed bet: pitch ref, armed budget, gate history
  scope/
    autopay/
      pay-form-toggle.md   # tasks (must-have / ~nice-to-have), hill position, status
      disable-on-customer.md
  hill/
    autopay.jsonl          # append-only hill snapshots → the "how it's moving" view
  log.jsonl                # run events: budget burn, gate crossings, breaker trips
```

A **pitch** carries the five ingredients in a fixed structure so both humans and agents can rely on it. A **scope** is a checklist plus a hill position. The **hill** file is append-only so history (the manager's killer feature — seeing what's _in motion vs. stuck_) is preserved.

---

## 9. The resource model: appetite, budget & circuit breaker

This is the heart of the agent reinterpretation, so it gets its own section.

**Appetite is declared while shaping and armed at the bet.** It is a budget triple — the run stops when _any_ leg is exhausted (whichever trips first):

```
appetite = { tokens: <ceiling>, iterations: <ceiling>, wallclock: <ceiling> }
```

Two presets keep the book's vocabulary:

- **Small Batch** — one scope, one agent session. Tight default caps.
- **Big Batch** — multiple scopes, possibly multiple sessions. Larger caps.

**The circuit breaker is the default, not the exception.** When a run exceeds its appetite, `shape` **halts** — it does _not_ silently extend. A blown budget means the shaping was wrong; the right response is to re-shape, not to throw more tokens at a bad approach. This is precisely the book's "cap the downside" — made literal and automatic.

**Extension follows the book's discipline.** The one rule for overriding the breaker maps beautifully onto the hill chart: **you may extend a run only if all remaining work is downhill** — every scope's dot is over the crest, no unknowns left. Any uphill scope at budget-exhaustion is an unsolved problem, which means more tokens are a gamble, not a finish. `shape` can enforce exactly this: `shape build --extend` is refused while any scope is still uphill.

---

## 10. Scale-adaptive design

Per the book's _Adjust to Your Size_: the **basic truths** (shape the work, cap the downside, make unknowns visible) hold at every scale; the **ceremony** is optional and grows with the team.

**Solo builder + agent fleet (default).** Wing it. No betting table — `shape bet` is a one-person decision. No mandatory cool-down. Phases can blur: shape a thing, build it, shape the next. The human alternates "hats" while one or several agents do the building. `shape` stays out of the way and just holds the guardrails (appetite, gates, hill).

**Small team + agents.** Turn on ceremony. `shape bet` becomes a reviewable betting table (multiple stakeholders, a recorded decision). Multiple bets run in parallel — a fleet of agents across scopes — and `shape status` aggregates the hill charts so a lead can see every run's motion at a glance. Cool-down becomes a shared, scheduled window.

The same `.shape/` files work both ways; scaling up is turning on flags, not migrating tools.

### New-product modes

For greenfield work the book's three modes carry over, and `shape` adjusts what it expects:

- **R&D mode** — exploratory spikes with a senior agent + human. Shaping is fuzzy, **no ship gate**, the goal is to learn the codebase's "shape" and lay load-bearing architecture. `shape build --spike` loosens validation and treats output as throwaway-by-default.
- **Production mode** — architecture settled; standard shape→bet→build with crisp gates; **parallelize** across agents/scopes. "Ship" = merge to main.
- **Cleanup mode** — pre-launch free-for-all; no shaping; agents fix edges continuously; the human makes final-cut decisions. `shape` drops ceremony and behaves like a bug-smash.

---

## 11. Agent integration (agent-agnostic)

`shape` never assumes a specific agent. `shape init` detects what's in the repo and writes the matching **adapter** — the instruction surface each agent reads (this is exactly why the repo's `.gitignore` already accommodates `.claude/`, `.cursor/`, `.aider*`, `.windsurf/`, `.continue/`, `.codeium/`, `.gemini/`, `.opencode/`, and Copilot instructions, while keeping the top-level `skills/` tracked):

- A shared **skill** in `skills/` teaches the Shape Up loop and the artifact format.
- Per-agent **adapters** (e.g. `CLAUDE.md`, `.cursorrules`, `copilot-instructions.md`) point the agent at `shape` commands and the `.shape/` files.
- The integration contract is intentionally low-tech: **the agent reads and writes files, and may call `shape`.** That keeps it portable across every present and future agent. An MCP server / richer integration is a later option, not a v1 dependency.

---

## 12. Example: a run, end to end

```console
$ shape pitch new autopay
  ◇ co-shaping "autopay" — drafting with your agent…
  ◇ agent: narrowed problem → "let payers auto-pay future invoices"
  ◇ agent: appetite proposed → Small Batch · breadboard drafted · 2 rabbit holes flagged
  → review .shape/pitch/autopay.md

$ shape derisk autopay
  ◇ agent (adversarial): "Does enabling Autopay also pay the current invoice?" → patched
  ◇ agent: flagged username/password management as scope risk → recommended NO-GO

$ shape pitch ready autopay         # ▸ human gate: shaped
  ✓ pitch valid (problem · appetite · solution · rabbit holes · no-gos)

$ shape bet autopay --appetite small        # ▸ human gate: bet
  ✓ bet 001 placed · circuit breaker armed { tokens, iters, 20m }

$ shape build
  ◇ agent oriented · proposed scopes: [Pay-Form Toggle*, Disable-on-Customer]
  ◇ get-one-piece-done → building "Pay-Form Toggle" (core · small · novel)
  ◇ hill: Pay-Form Toggle ▲ uphill 35%

$ shape status
  bet 001 · Small Batch · budget 41% spent
  ● Pay-Form Toggle      downhill 80%   (moving)
  ● Disable-on-Customer  uphill   10%   ⚠ unchanged 2 updates — possible stuck

$ shape accept slice                 # ▸ human gate: first slice demoed & accepted
$ shape accept done                  # ▸ human gate: finished
$ shape ship
  ✓ deployed · changelog drafted · 3 discovered ~nice-to-haves left for cool-down
```

---

## 13. Design tenets

1. **The human judges; the agent works; the CLI referees.** Gates and budgets are non-negotiable; everything else is suggestion.
2. **Bound everything.** Appetite bounds cost, scope bounds context, the breaker bounds the downside.
3. **Make uncertainty visible.** Agents won't raise their hand — the hill chart does it for them.
4. **Files over databases.** Git-native, diffable, agent-readable, no server.
5. **Agent-agnostic, always.** Adapters, never lock-in.
6. **Ceremony is optional.** Solo by default; scale up by turning things on.
7. **Stay rough on purpose.** Shaping leaves room for the agent to find the real implementation — the agent equivalent of "no wireframes too early."
8. **Keep the slate clean.** One bet at a time; post-run ideas re-enter as raw ideas.

---

## 14. v1 scope & roadmap

v1 targets the **full methodology** mapped above; the order below is about sequencing, not cutting.

**Milestone 1 — Artifacts & shaping.** `init`, `pitch new/list/show/ready`, the pitch file format + 5-ingredient validation, one agent adapter (the repo's primary agent). Co-shaping and `derisk`.

**Milestone 2 — Betting & budgets.** `bet`, the appetite/budget model, the armed circuit breaker, `bets`.

**Milestone 3 — Building & visibility.** `build` orchestration, `scope` mapping, the terminal **hill chart** + history, `status` with stuck-dot detection, budget metering, `stop`, accept gates.

**Milestone 4 — Ship, cool-down, scale.** `ship` + move-on checklist, `cooldown`, multi-bet parallel status, the team-mode betting table, new-product modes (`--spike`).

**Later (post-v1):** MCP / richer agent integration, parallel-agent fleet orchestration, budget analytics across runs.

---

## 15. Open questions

- **Budget calibration.** What are sane default caps for Small vs. Big Batch, and should they auto-tune from observed run history?
- **Hill self-reporting trust.** Agents are overconfident — how much should `shape` corroborate a self-reported hill position against actual activity (commits, test runs, token burn) before trusting it?
- **Where the agent runs.** Does `shape build` invoke the agent directly, or emit a kickoff brief the human pastes into their agent session? (Likely both, via adapter.)
- **Gate friction vs. autonomy.** Four human gates is safe but can feel heavy for a solo builder — should gates be collapsible to a single end-of-run review in solo mode?
- **Concurrency model.** How do parallel agents on sibling scopes avoid stepping on each other — worktrees, file locks, or scope-level ownership?

---

## 16. Glossary

Same terms as the book ([§ Glossary](shapeup/4.5-appendix-06.md)), re-pointed for agents:

- **Appetite** — the resource budget (tokens · iterations · wall-clock) a raw idea is worth. Declared while shaping.
- **Bet** — committing one shaped pitch to an agent run with the budget armed.
- **Circuit breaker** — automatic halt when a run exceeds its appetite; extend only if all work is downhill.
- **Co-shaping** — the agent drafts the pitch; the human edits and approves.
- **Gate** — a human approval that lets an artifact advance a phase. The core safety property.
- **Hill chart** — per-scope uphill (figuring out) → downhill (executing) status over time; surfaces stuck agents.
- **No-go** — something deliberately excluded to fit the appetite or keep the problem tractable.
- **Pitch** — the shaped artifact: problem · appetite · solution · rabbit holes · no-gos.
- **Rabbit hole** — an unknown an agent could grind on; removed during shaping, not during building.
- **Run** — one bounded build session against a placed bet (the agent-era "cycle").
- **Scope** — an independently completable, context-window-sized slice of a project.
- **Scope hammering** — cutting `~` nice-to-haves to fit the appetite; the antidote to gold-plating.

```

```
