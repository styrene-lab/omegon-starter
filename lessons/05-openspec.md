# Lesson 05 — OpenSpec

> **Mode:** Full (`omegon`, not `om`). If you're in slim mode, type `/warp` to switch.

**Goal:** Learn spec-driven development with Omegon's OpenSpec lifecycle — write the spec first, then implement to match it.

## Context

OpenSpec is a structured workflow for making changes:

```
proposal → spec → design → implement → verify → assess → archive
```

Each change gets its own directory under `openspec/changes/` with:
- A **proposal** (what and why)
- **Specs** (Given/When/Then scenarios that define done)
- A **design** (how to implement)
- **Tasks** (subtask breakdown)

When the work is done and specs pass, the change is **assessed** and **archived** into the baseline.

## Exercises

### 5.1 — Propose a change

Let's spec out due date support for Trellis. Propose it formally:

- `/opsx:propose` or "Propose an OpenSpec change: add due date support to Trellis"

Watch Omegon create a change directory with a `proposal.md` capturing:
- Problem statement: tasks have no time dimension
- Motivation: users need to track deadlines
- Scope: what's in and what's explicitly out

**What you're learning:** Proposals force you to articulate _why_ before _how_. They're the entry gate for non-trivial changes.

### 5.2 — Write specs

Ask Omegon to write specs for the change:

- "Write specs for the due date feature"

Specs are Given/When/Then scenarios:
```
Given a task created with --due 2026-01-15
When I run trellis list
Then the task shows its due date

Given a task with due date 2026-01-01
When the current date is 2026-01-02
Then the task is shown with an overdue indicator
```

**What you're learning:** Specs define behavior concretely. They become acceptance criteria — the change isn't done until every scenario passes.

### 5.3 — Design the implementation

- "Design the implementation for the due date change"

Omegon creates `design.md` with:
- Data model changes (new field on Task)
- CLI changes (new `--due` flag)
- Display changes (overdue indicator in list output)
- Migration (existing tasks without due dates still work)

**What you're learning:** The design step bridges spec and code. You decide the approach before writing code.

### 5.4 — Implement (or don't)

You can ask Omegon to implement the change, or leave it as a spec exercise. The point of this lesson is the lifecycle, not the feature.

If you want to see it through:
- "Implement the due date spec"
- "Run the tests to verify"
- `/opsx:assess spec` — verify all scenarios pass

**What you're learning:** The full lifecycle — from "we should add this" through a concrete spec to working, verified code.

### 5.5 — Browse the OpenSpec state

- "Show me the OpenSpec status"
- "What changes are in progress?"

**What you're learning:** OpenSpec state is queryable. You can see what's proposed, specced, in progress, and archived.

## What you learned

- **Spec-first development**: acceptance criteria before code
- **Change lifecycle**: proposal → spec → design → implement → verify → assess → archive
- **Given/When/Then**: concrete scenarios that define "done"
- **Design bridge**: between spec and code
- **Assessment**: formal verification that specs are satisfied

**Next:** Lesson 06 — Postures & Effort
