# Design: Add structured schedule receipts

## Context

`servers/valence/src/tests/core_plugin_sets.rs` inspects Bevy schedules by rendering DOT graphs and searching strings. The accepted schedule specs already require reviewable schedule evidence, but the current evidence shape is optimized for tests rather than deterministic receipts.

## Decisions

### 1. Receipts record selected facts, not full graphs

**Choice:** A receipt names the command, app/plugin configuration, schedule label, expected and observed sets, expected and observed systems where selected, disabled-plugin comparisons, ambiguity mode, and pass/fail diagnostics.

**Rationale:** Reviewers need stable facts, not a full graph serialization for every change.

### 2. Bevy inspection stays in the shell

**Choice:** The shell queries Bevy schedules and extracts raw facts. A pure deterministic normalization/checking function consumes in-memory facts and expected facts to produce receipt rows and diagnostics.

**Rationale:** Functional core tests should not need a Bevy `World`, filesystem, clocks, or process execution.

### 3. DOT remains optional

**Choice:** DOT graph output can remain available for debugging, but task-cited evidence should prefer structured receipt JSON/Markdown when possible.

**Rationale:** DOT graphs are useful but noisy and sensitive to formatting changes.

### 4. Receipts must prove negative cases

**Choice:** Negative fixtures should cover unknown schedule, missing expected set, unexpected installed plugin/system, and ambiguity-mode regressions where practical.

**Rationale:** A receipt format is only useful if it fails clearly when important facts drift.

## Risks / Trade-offs

- Bevy APIs may not expose every private ordering relation in a stable way; receipts should start with selected schedules and sets.
- Adding receipt generation to tests could make logs noisy; keep output opt-in or fixture-local unless evidence is requested.
- Structured facts may need stable display names for systems and sets; normalize names in one helper to avoid ad hoc string matching.
