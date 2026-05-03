# Lesson 08 — Extensions

> **Mode:** Full (`omegon`, not `om`)

**Goal:** Understand how Omegon's extension system adds capabilities through isolated processes.

## Context

Extensions are separate binaries that communicate with Omegon via **JSON-RPC 2.0 over stdin/stdout**. They add tools, widgets, and cross-project intelligence without risking the harness itself — if an extension crashes, Omegon keeps running.

Omegon ships with three first-party extensions:
- **Scribe** — forge integration (GitHub/Forgejo issues, PRs, timeline)
- **Vox** — communication connectors (email, Signal, Slack, Discord)
- **Scry** — local image generation (FLUX, SDXL via ComfyUI)

## Exercises

### 8.1 — List installed extensions

- "What extensions are installed?"
- `/help extensions`

This shows which extensions are loaded, their health status, and what tools they provide.

**What you're learning:** Extensions are discovered at startup. Each one declares its tools via a `get_tools` RPC call. If an extension isn't responding, Omegon disables it gracefully.

### 8.2 — Explore extension tools

If you have Scribe installed:

- "What tools does Scribe provide?"
- "Show me the recent timeline for this repo"

If you don't have any extensions installed, that's fine — this exercise shows you what the system looks like and how extensions surface their capabilities.

**What you're learning:** Extension tools appear alongside built-in tools. The agent uses them the same way — the RPC boundary is invisible during normal use.

### 8.3 — Extension safety model

Ask Omegon about extension safety:

- "What happens if an extension crashes?"
- "How does Omegon isolate extensions?"

Key points:
- **Process isolation** — extensions can't corrupt Omegon's memory or state
- **Crash recovery** — EOF detection, graceful disable after 3+ crashes per session
- **RPC timeouts** — unresponsive extensions return errors, not hangs
- **SDK version gating** — version mismatch blocks installation entirely

**What you're learning:** The safety model is a key differentiator. Extensions are powerful but can't break things — they're tenants, not roommates.

### 8.4 — Extension manifests

Extensions declare themselves via `manifest.toml`:

```toml
[extension]
name = "scribe-rpc"
version = "0.4.0"
sdk_version = "0.15"
description = "Forge integration — issues, PRs, timeline"
binary = "scribe-rpc"
ping_method = "get_tools"
```

Ask Omegon:
- "Where are extensions installed?"
- "What does an extension manifest look like?"

**What you're learning:** Extensions are discoverable, versioned, and self-describing. The manifest is the contract between extension and harness.

## What you learned

- **Extensions** are isolated processes communicating via JSON-RPC
- **Safety**: crash isolation, timeout handling, version gating
- **Discovery**: tools and widgets registered at startup
- **First-party**: Scribe (forge), Vox (comms), Scry (image generation)
- **Manifest**: self-describing packages with SDK version contracts

## What's next

You've completed the Omegon Starter tutorial. You now know:

1. **Core tools** — reading, editing, running commands
2. **Memory** — durable project facts across sessions
3. **Skills** — passive domain conventions
4. **Design tree** — exploration and decision tracking
5. **OpenSpec** — spec-driven development lifecycle
6. **Postures** — explicit cost/capability control
7. **Cleave** — parallel execution with isolation
8. **Extensions** — safe, isolated capability plugins

From here:
- Start a real project and use these features for actual work
- Explore `/help all` for the full command reference
- Try the **Auspex** browser dashboard: `/auspex open`
- Read the docs at https://omegon.styrene.io/docs/
