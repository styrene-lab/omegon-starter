# Lesson 07 — Cleave

> **Mode:** Full (`omegon`, not `om`). If you're in slim mode, type `/warp` to switch.
> **Requires:** A Victory-tier model or better for child agents.

**Goal:** Learn how Omegon decomposes large tasks into parallel child agents, each working in an isolated git worktree.

## Context

When a task is too large for a single pass — multiple files, multiple concerns, risk of drift — Omegon can **cleave** it into parallel subtasks. Each child agent:

- Gets its own **git worktree** (isolated copy of the repo)
- Works on its own **branch**
- Reports back with a **status** and **merge-ready branch**
- Can fail independently without blocking siblings

The parent orchestrator spawns the children, monitors progress, merges results, and handles conflicts.

## Exercises

### 7.1 — Identify a cleave-worthy task

Trellis is missing several independent features. Ask Omegon whether this set is worth cleaving:

- "I want to add three features to Trellis: (1) a `stats` subcommand showing task counts and completion rate, (2) an `edit <id> <new-title>` subcommand for renaming tasks, and (3) colored priority indicators in list output. Should we cleave this?"

Watch Omegon assess:
- Are the changes independent? (Yes — different concerns)
- Is each one non-trivial enough? (Moderate — worth parallelizing)
- Would sequential work risk drift? (Three features touching main.rs — merge coordination helps)

**What you're learning:** Cleave isn't for everything. It's for work that decomposes into independent, non-trivial units.

### 7.2 — Execute a cleave

- `/cleave` or "Cleave the three features"

Watch Omegon:
1. Decompose into 3 child tasks
2. Create worktrees for each
3. Spawn child agents
4. Report progress as each completes

Each child exits with a code:
- `0` — done, branch is merge-ready
- `1` — error, needs attention
- `2` — exhausted context, partial work on branch
- `3` — timed out

**What you're learning:** Parallel execution with isolation. Each child can't corrupt the others' work.

### 7.3 — Review and merge

After cleave completes:

- "Show me what each child did"
- "Merge the cleave results"

Omegon merges the branches, handling conflicts. If two children touched the same code, you'll see the conflict and can resolve it with the agent's help.

**What you're learning:** The orchestration lifecycle — spawn, monitor, merge. The parent agent handles coordination so you don't manually juggle branches.

### 7.4 — When not to cleave

Ask Omegon about a task that shouldn't be cleaved:

- "Should I cleave adding a `--verbose` flag to list?"

It should say no — that's a small, localized change. Cleave adds overhead (worktree creation, branch management, merge). Only worth it when parallelism saves more time than coordination costs.

**What you're learning:** Judgment. Cleave is powerful but not free. Good engineering is knowing when to reach for the big tool.

## What you learned

- **Cleave** decomposes work into parallel, isolated child agents
- **Worktrees**: each child gets its own copy of the repo
- **Exit codes**: structured reporting (done, error, exhausted, timeout)
- **Merge orchestration**: parent handles branch merging and conflicts
- **Judgment**: cleave when work is naturally parallel and non-trivial

**Next:** Lesson 08 — Extensions
