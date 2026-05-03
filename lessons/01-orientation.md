# Lesson 01 — Orientation

**Goal:** Get comfortable with Omegon's core tools — reading code, making edits, and running commands.

## Context

You're looking at **Trellis**, a small CLI task tracker written in Rust. It works, but has a bug and some missing features. In this lesson you'll explore the codebase, find and fix a bug, and wire up a feature that's already implemented but not exposed.

## Exercises

### 1.1 — Read the codebase

Ask Omegon to explore the project:

- "Read the project and explain how it works"
- "What does the Task struct look like?"
- "How does the store persist data?"

**What you're learning:** Omegon's `read` and `codebase_search` tools. The agent navigates files, understands structure, and explains code without you pointing at specific lines.

### 1.2 — Find the bug

There's a bug somewhere in the task lifecycle. The test suite has a clue — one test is ignored.

Ask Omegon:
- "Run the tests"
- "Why is one test ignored? Run it and see what happens"
- "Find and fix the bug"

Don't peek at the source first — let the agent trace from the test failure to the root cause.

**What you're learning:** Omegon's `bash` tool (running cargo), `edit` tool (surgical code changes), and how it reasons about bugs from test evidence.

### 1.3 — Wire up the search subcommand

There's a `search` method on `Store` that works but isn't connected to a CLI subcommand. The TODOs in `main.rs` mark where it should go.

- "The search method in Store isn't connected to a CLI command. Add a `trellis search <query>` subcommand."

Then test it:
- "Add a couple test tasks, then search for one"

**What you're learning:** Omegon reading existing code, understanding intent from context, and making edits that match existing patterns.

### 1.4 — Run it for real

Try the full workflow — ask Omegon to run these, or do it yourself:

```
cargo run -- add "Learn Omegon" --priority high
cargo run -- add "Read the docs" --priority low
cargo run -- list
cargo run -- search "Omegon"
cargo run -- done 1
cargo run -- list
```

**What you're learning:** The full loop — build, run, verify. Omegon can drive CLI tools and interpret their output.

## What you learned

- **Reading code**: project structure, search, explanation
- **Editing code**: surgical changes that fit existing style
- **Running commands**: cargo build, test, run — with real output
- **Bug hunting**: tracing from test failure → root cause → fix

**Next:** Lesson 02 — Project Memory
