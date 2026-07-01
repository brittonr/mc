## Context

The archived furnace behavior-card and selected-row core work established a bounded standard-furnace path:

- `docs/furnace-smelting-behavior-card.md` says wiki pages guide vocabulary only and require extracted data plus Paper/vanilla receipts before behavior claims.
- `docs/furnace-smelting-selected-row-core.md` says the current core uses local fixture rows only and names extracted Java Edition 1.20.1 recipe/fuel fixtures as next required evidence.
- `cairn/specs/vanilla-composable-plugins/spec.md` requires selected-row evidence to reject broad vanilla parity, all-recipe breadth, smoker/blast-furnace behavior, hoppers, XP, recipe book, chunk-unload behavior, public-server safety, and production readiness.

## Decisions

### 1. Plan a selected-row data fixture contract before broader evidence

**Choice:** The next implementation should add a contract for one selected standard-furnace recipe row and one selected fuel row scoped to Java Edition 1.20.1 / protocol 763.

**Rationale:** This is the smallest missing evidence rail between the local pure core and a future Paper/vanilla parity row. It avoids all-recipe breadth while making data provenance and validation reviewable.

### 2. Validate fixture shape before using it in the core

**Choice:** The follow-on implementation must include a focused validator with positive and negative cases for target scope, selected-row membership, recipe/fuel row shape, malformed rows, missing rows, unsupported furnace kinds, and non-claim text.

**Rationale:** The current core already treats malformed recipe and fuel rows as typed errors. A fixture validator should fail before bad rows reach the core.

### 3. Keep Paper parity and Valence shell work deferred

**Choice:** This target stops at the data fixture contract. Paper/vanilla receipts and Valence Bevy/ECS shell design are named as follow-on work, not included here.

**Rationale:** Data extraction, live parity, and runtime shell integration have different evidence and schedule risks. Keeping this Cairn as a contract package preserves reviewability.

## Risks / Trade-offs

- A contract-only Cairn does not deliver target-version fixture bytes yet; it prevents ambiguity about what the fixture implementation must prove.
- Selecting one standard-furnace row may feel narrow, but it is intentionally scoped to unblock a single parity row before all-recipe breadth.
- If future extracted data format differs from this contract, the implementation Cairn must update the spec with a clear migration and validation reason.
