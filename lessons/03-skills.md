# Lesson 03 — Skills

**Goal:** Understand how Omegon's built-in skills provide domain-specific conventions without tool calls.

## Context

Skills are bundled knowledge packs — markdown files with TOML frontmatter that inject context-aware guidance into the agent's prompts. They're not tools (no API calls); they're conventions that shape how the agent behaves.

Omegon ships with skills for: **git**, **rust**, **python**, **typescript**, **oci**, **openspec**, **security**, **vault**.

## Exercises

### 3.1 — See active skills

Ask Omegon what skills are active:

- "What skills are loaded for this project?"

Since this is a Rust project with a git repo, the **rust** and **git** skills should be active. These inject conventions like:
- Rust: edition 2021, cargo test before committing, clippy, rustfmt
- Git: conventional commit messages (`feat:`, `fix:`, `refactor:`)

**What you're learning:** Skills are auto-detected based on project content (Cargo.toml triggers Rust, .git triggers Git).

### 3.2 — Git skill in action

If you completed lesson 01, commit those changes. If you skipped it, make a small change first (e.g., update the description in `Cargo.toml`):

- "Commit my changes"

Watch the commit message — it should follow conventional commit format (e.g., `fix: set completed_at timestamp in Task::complete()`). That's the git skill at work, not explicit instructions.

**What you're learning:** Skills shape behavior implicitly. You don't invoke them — they steer the agent toward project-appropriate conventions.

### 3.3 — Rust skill in action

Ask Omegon to write a test for a feature that already exists but lacks coverage:

- "Write tests for the Store — cover add, list, complete, remove, and search"

Watch for:
- Tests placed in the right location (tests/ or inline #[cfg(test)])
- Idiomatic assertions
- Running `cargo test` after writing them

**What you're learning:** The Rust skill encodes testing idioms, file organization conventions, and the test-after-change workflow.

### 3.4 — Security skill

Ask Omegon to review the code for security concerns:

- "Are there any security concerns with how Trellis stores data?"

The security skill provides OWASP awareness and data handling guidance. For Trellis, it might flag that `trellis.json` is world-readable, or that there's no input length validation.

**What you're learning:** Skills include domain expertise beyond formatting — security awareness, container best practices, dependency audit patterns.

## What you learned

- **Skills are passive context**, not tools — they shape behavior without explicit invocation
- **Auto-detection**: skills activate based on project content
- **Convention enforcement**: commit formats, test workflows, language idioms
- **Domain expertise**: security awareness, container practices, etc.
- **Composable**: multiple skills active simultaneously

**Next:** Lesson 04 — Design Tree
