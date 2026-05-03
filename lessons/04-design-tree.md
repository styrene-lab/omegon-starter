# Lesson 04 — Design Tree

> **Mode:** Full (`omegon`, not `om`). If you're in slim mode, type `/warp` to switch.

**Goal:** Learn how Omegon tracks design exploration and decisions as first-class artifacts.

## Context

The **design tree** is a structured journal for design thinking — alternatives considered, questions explored, decisions made, and why. Each node has a lifecycle:

```
seed → exploring → decided → implementing → implemented
```

Design nodes live outside the code. They capture the _why_ behind decisions, not just the _what_. They survive refactors, rewrites, and branch changes.

## Exercises

### 4.1 — Create your first design node

Trellis stores tasks in a flat JSON file. Is that the right choice? Let's explore:

- "Create a design note exploring storage alternatives for Trellis — flat JSON vs SQLite vs directory-per-task"

Watch Omegon create a design node with status `seed`.

**What you're learning:** The `design_tree_mutate` tool. A `seed` is a question that hasn't been explored yet — just planted.

### 4.2 — Explore the design space

Ask Omegon to flesh out the exploration:

- "Explore the storage design note — what are the pros and cons of each approach for a small CLI tool?"

The agent should advance the node to `exploring` and capture:
- Trade-offs for each option
- What matters for Trellis specifically (simplicity, portability, no dependencies)
- Rejected alternatives and why

**What you're learning:** Exploration is iterative. The node accumulates context as you think through the problem.

### 4.3 — Make a decision

Settle on flat JSON (it's the right call for a tiny CLI):

- "Decide on the storage question — flat JSON is the right choice for Trellis. Record why."

The node should advance to `decided` with a clear rationale.

**What you're learning:** `decided` nodes capture rationale, not just the choice. When someone asks "why flat JSON?" six months from now, the answer is in the design tree.

### 4.4 — View the design tree

- `/design` or "Show me the design tree"
- `/dash` — the dashboard shows a compact status view

**What you're learning:** The design tree is browsable and queryable — a living record of design thinking.

### 4.5 — Plant a second seed

Create a node for a feature question without resolving it:

- "Create a design note: should Trellis support tags on tasks?"

Leave it as a `seed`. Not every question needs an answer right now.

**What you're learning:** Seeds are open questions worth tracking. The tree captures what's been asked, what's been explored, and what's been decided.

## What you learned

- **Design nodes** track exploration, not just decisions
- **Lifecycle**: seed → exploring → decided → implementing → implemented
- **Rationale capture**: decisions record _why_, not just _what_
- **Living artifact**: survives code changes, queryable, browsable
- **Low friction**: plant a seed when a question comes up, explore it when ready

**Next:** Lesson 05 — OpenSpec
