# Lesson 06 — Postures & Effort

**Goal:** Understand how Omegon gives you explicit control over inference cost, capability, and reasoning depth — and see the difference it makes.

## Context

Most AI tools hide their inference decisions behind "smart routing." Omegon rejects this — you get three explicit axes of control:

1. **Capability Tier** — which model class (`local` → `retribution` → `victory` → `gloriana`)
2. **Thinking Level** — reasoning effort (`off` → `minimal` → `low` → `medium` → `high`)
3. **Context Class** — token budget (`squad` 128K → `maniple` 272K → `clan` 400K → `legion` 1M+)

**Posture presets** bundle these axes:

| Posture | Model | Thinking | Context | Use Case |
|---------|-------|----------|---------|----------|
| **Explorator** | local | minimal | squad | Quick recon |
| **Fabricator** | sonnet | low | maniple | Standard dev |
| **Architect** | sonnet/opus | medium | clan | Complex design |
| **Devastator** | opus | high | legion | Exhaustive analysis |

## Exercises

### 6.1 — Check current posture

- `/whoami` — shows provider, model, context class, thinking level

Look at the TUI footer too — it always displays the active provider and model. No hidden routing.

**What you're learning:** Omegon is always honest about what's running.

### 6.2 — Posture-appropriate code review

Do the same task at two different effort levels and compare:

First, at a lower effort:
- `/effort Substantial`
- "Review store.rs for potential issues"

Note the response — likely hits the obvious points (error handling, file locking).

Now at a higher effort:
- `/effort Lethal`
- "Review store.rs for potential issues"

Compare: the higher effort should produce deeper analysis — race conditions, atomicity of write, ID reuse after remove, edge cases in the sort.

**What you're learning:** Posture isn't just about cost. Different effort levels produce different depths of analysis. Not every task needs maximum resources.

### 6.3 — Reasoning depth at different tiers

Ask the same architectural question at two different effort levels:

First:
- `/effort Substantial`
- "If Trellis needed to support 100,000 tasks, what would need to change?"

Then:
- `/effort Lethal`
- "If Trellis needed to support 100,000 tasks, what would need to change?"

Compare: the lower tier gives a reasonable surface answer. The higher tier should go deeper — file locking, memory-mapped I/O, indexed storage, ID allocation strategy, incremental serialization.

**What you're learning:** Matching capability to task complexity. A quick sanity check doesn't need Opus. A system architecture question does.

### 6.4 — The effort tiers

The `/effort` command sets a single knob controlling the local-vs-cloud ratio:

| Tier | Cloud % | When to use |
|------|---------|-------------|
| Servitor | 0% | Offline, local-only |
| Average | 0% | Local with minimal reasoning |
| Substantial | ~30% | Normal dev work |
| Ruthless | ~45% | Important changes |
| Lethal | ~65% | Complex multi-file work |
| Absolute | ~85% | Critical path |
| Omnissiah | 100% | Everything through the best model |

Try: `/effort Ruthless` — a good default for tutorial work.

**What you're learning:** The seven tiers give you a simple mental model. You don't need to think about model names — just "how important is this task?"

## What you learned

- **Three axes**: capability tier, thinking level, context class — independent controls
- **Postures**: named presets that bundle the axes
- **Effort tiers**: single knob for local-vs-cloud ratio
- **Provider honesty**: footer always shows exactly what's running
- **Cost/quality trade-off**: visible and explicit, not hidden

**Next:** Lesson 07 — Cleave
