# Design: Close mc-compat dry-run coverage gaps

## Context

The current manifest distinguishes dry-run-covered rows from rows that are maintained but excluded because they are backed by historical live receipts or checker-gated paired receipts. The harness should make that distinction executable: maintained rows should normally have deterministic local shape coverage, while exceptions should be visible and time-bounded.

## Decisions

### 1. Treat dry-run coverage as part of maintained status

**Choice:** A maintained scenario SHOULD have a deterministic dry-run receipt-shape check unless a waiver records why that is not currently possible.

**Rationale:** Maintained should mean the row participates in cheap regression checks, not only that historical evidence exists.

### 2. Preserve bounded non-claim wording

**Choice:** New dry-run receipts will keep existing explicit non-claims and will not upgrade fixture evidence into live parity evidence.

**Rationale:** Shape checks prove schema/oracle wiring. They do not prove semantics.

### 3. Use pure manifest coverage evaluation

**Choice:** Add or extend a pure checker core that classifies rows as covered, waiver-backed, or invalid from manifest data. The CLI/Nix shell only reads files and reports diagnostics.

**Rationale:** Coverage policy can be tested with in-memory positive and negative fixtures.

### 4. Convert exclusions incrementally

**Choice:** Start with scenarios that already share existing runner behavior and only need deterministic fixture output. Leave paired-reference or historical rows behind explicit waivers when conversion would imply new evidence semantics.

**Rationale:** This avoids overstating dry-run evidence while still reducing gaps.

## Risks / Trade-offs

- Adding wrappers for historical rows can look like new semantic evidence; mitigate with receipt non-claims and checker diagnostics.
- Waivers can become permanent; mitigate by requiring owner, reason, and next action in the manifest/checker output.
- Wrapper duplication can grow; mitigate by generating common wrapper metadata from scenario manifest data where practical.
