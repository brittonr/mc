# Design: Survival sign editing live parity

## Context

The existing sign block-entity row observes a configured persisted sign payload. This change defines an interaction row for opening the editor, submitting text, and observing accepted state.

## Decisions

### 1. Reuse but do not merge sign persistence evidence

**Choice:** The live editing row may reuse sign fixture setup but records separate open/update/acceptance metrics.

**Rationale:** Persisted sign observation and sign editing UI are distinct claims.

### 2. Require client and server correlation

**Choice:** Receipts must include client open/update milestones and server acceptance with the same position, side, and text payload.

**Rationale:** Sign editing can appear client-side without server acceptance; paired correlation prevents false promotion.

### 3. Keep formatting breadth out of scope

**Choice:** Text formatting, all sign variants, side permutations, and arbitrary NBT remain non-claims.

**Rationale:** The first live row should prove one exact payload only.
