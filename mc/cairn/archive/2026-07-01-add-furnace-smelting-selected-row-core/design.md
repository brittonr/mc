## Context

`docs/furnace-smelting-behavior-card.md` requires a pure deterministic core before any Valence shell work. The accepted `vanilla-composable-plugins` spec already requires wiki-derived behavior decisions to live in pure cores and reserves Bevy/ECS mutation for thin shells.

This change implements only a local selected-row standard-furnace core. It uses in-memory fixture tables that stand in for future Java Edition 1.20.1 extracted recipe/fuel data. It deliberately does not implement Valence integration or promote vanilla parity.

## Decisions

### 1. Implement the core as a focused Rust checker script

**Choice:** Add `tools/check_furnace_smelting_core.rs` as a repo-owned Rust script with a pure core and a thin CLI self-test shell.

**Rationale:** The repository does not yet have a Valence plugin crate for vanilla furnace behavior. A focused script lets reviewers inspect and test deterministic state transitions without starting Valence, connecting clients, reading files, or introducing runtime dependencies.

### 2. Keep the first slice standard-furnace and selected-row only

**Choice:** Support only `FurnaceKind::Standard` and one selected target recipe/fuel row in tests.

**Rationale:** This proves the core shape and error behavior while preserving stop conditions for all-recipe breadth, smoker, blast furnace, hoppers, XP, recipe book, chunk unload, data packs, and default plugin membership.

### 3. Make validation pure and explicit

**Choice:** The core validates malformed recipe and fuel rows, unsupported furnace kinds, blocked outputs, missing recipes, and missing fuel through typed transitions or typed errors.

**Rationale:** Positive and negative tests can exercise behavior without mocks or external services. This keeps the functional core independent from future ECS shells and data-loading boundaries.

## Risks / Trade-offs

- The script is not a Valence plugin API; a future implementation Cairn must port or promote the core into the appropriate crate before runtime use.
- Fixture recipe/fuel rows are local unit examples, not extracted 1.20.1 data.
- Tick semantics are local selected-row semantics only and remain non-authoritative until compared with extracted data and Paper/vanilla receipts.
