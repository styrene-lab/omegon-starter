# Lesson 02 — Project Memory

**Goal:** Learn how Omegon remembers things about your project across sessions.

## Context

Most AI tools forget everything when you close the chat. Omegon maintains **project memory** — durable facts about architecture, decisions, constraints, and patterns that persist across sessions.

Memory is stored in `ai/memory/facts.jsonl` — a BM25-indexed fact store with typed sections and decay scoring. Facts you use get reinforced; stale facts fade.

## Exercises

### 2.1 — Store your first facts

After exploring the codebase in lesson 01, you have context worth preserving. Ask Omegon to remember it:

- "Remember that Trellis stores tasks in a flat JSON file called trellis.json"
- "Remember that priority sorting puts high-priority tasks first in list output"
- "Remember that Task IDs are sequential and never reused after removal"

**What you're learning:** The `memory_store` tool. Facts are categorized into sections: Architecture, Decisions, Constraints, Known Issues, Patterns & Conventions, Specs.

### 2.2 — Recall facts

Now pretend you're starting fresh. Ask:

- "What do you remember about how Trellis stores data?"
- "Recall everything about task IDs"
- `/memory recall "storage"`

**What you're learning:** The `memory_recall` tool — BM25 search over stored facts. Facts that get recalled have their decay timers reset, so useful facts stick around.

### 2.3 — Query the memory system

Try structured browsing:

- `/memory query` — see all facts, grouped by section
- "What architecture facts do you have?"
- "Are there any known issues stored in memory?"

**What you're learning:** The difference between `recall` (search by keyword) and `query` (browse by section). Both are useful in different situations.

### 2.4 — Memory-informed work

Now ask Omegon a question that requires combining multiple stored facts:

- "Based on what you know about Trellis, what would break if we changed the storage format from JSON to a binary format?"

Watch for it recalling stored facts — how the store works, what the data model looks like, what depends on the JSON format — and synthesizing them into a coherent answer.

**What you're learning:** Memory isn't just storage — it actively informs the agent's reasoning. Omegon recalls relevant facts before answering, so its analysis is grounded in what it already knows about the project.

## What you learned

- **Storing facts**: typed sections, natural language, automatic categorization
- **Recalling facts**: keyword search with BM25 scoring
- **Querying facts**: browsing by section
- **Memory-informed work**: facts guide the agent's reasoning, not just answers
- **Decay**: facts that aren't recalled fade over time — the memory stays relevant

**Next:** Lesson 03 — Skills
