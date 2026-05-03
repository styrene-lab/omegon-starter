# Omegon Starter

An interactive tutorial for learning [Omegon](https://omegon.styrene.io) — the systems engineering agent harness.

You learn by doing real work on a real codebase: a small Rust CLI task tracker called **Trellis**. Omegon acts as your tutor, walking you through its features while you build, fix, and extend the project.

## Prerequisites

- [Omegon](https://omegon.styrene.io/docs/) installed
- [Rust](https://rustup.rs/) toolchain (for building Trellis)
- At least one inference provider configured (run `omegon login` if you haven't)

> **Note:** The sample project is written in Rust, but you don't need deep Rust knowledge to follow along. The code is intentionally simple — if you can read C-like syntax, you'll be fine. Omegon explains what it's doing as it works.

## Quick start

```bash
git clone https://github.com/styrene-labs/omegon-starter.git
cd omegon-starter
omegon
```

Use `omegon` (not `om`) to start in **full mode** — lessons 04-08 require features only available in full mode (design tree, OpenSpec, cleave, extensions). You can always switch modes with `/warp`.

When Omegon opens, say **"start the tutorial"** or just start hacking.

## What you'll learn

| # | Topic | You'll do | Mode |
|---|-------|-----------|------|
| 01 | Orientation | Read code, find and fix a bug, wire up a feature | any |
| 02 | Memory | Store and recall project facts across sessions | any |
| 03 | Skills | See how built-in conventions shape agent behavior | any |
| 04 | Design Tree | Explore design decisions as structured artifacts | full |
| 05 | OpenSpec | Write specs first, implement to match | full |
| 06 | Postures | Control inference cost, capability, and reasoning | any |
| 07 | Cleave | Decompose work into parallel isolated agents | full |
| 08 | Extensions | Understand the extension system and safety model | full |

## The project: Trellis

A CLI task tracker with intentional learning opportunities:

- **A bug** to find and fix (lesson 01)
- **An unwired feature** to connect (lesson 01)
- **Missing tests** to write (lesson 03)
- **Design questions** to explore (lesson 04)
- **A spec** to write and fulfill (lesson 05)
- **Three features** to build in parallel (lesson 07)

```
src/
  main.rs    — CLI entry point and subcommand dispatch
  task.rs    — Task struct and Priority enum
  store.rs   — JSON file persistence and CRUD
  lib.rs     — library re-exports
tests/
  task_test.rs — unit tests (one intentionally ignored)
```

## Navigation

- **"start the tutorial"** — begin at lesson 01
- **"lesson N"** or **"next lesson"** — jump between lessons
- **"where am I?"** — check your progress
- **"reset the tutorial"** — clear progress tracking
- Or ignore the lessons and just build — Omegon adapts

## Provider requirements

Lessons 01-06 work with any provider, including local models (Ollama). Lesson 07 (cleave) requires a Victory-tier model or better (Sonnet, Opus) for child agents. Lesson 08 works with any provider but is more useful if you have extensions installed.

## License

MIT
