# Design: Stevenarella world functional core

## Context

World parsing and storage are central to client compatibility. The current module has a mix of pure decisions and mutation-heavy storage/rendering code. The first target is to extract deterministic decisions around dimension bounds and chunk layout because those directly affect protocol 763 and 1.20.1 behavior.

## Decisions

### 1. Start with dimension and chunk layout cores

**Choice:** Extract functions that choose dimension bounds from codec/name inputs, derive expected chunk section counts, and validate chunk/light/biome data shapes before mutating world storage.

**Rationale:** These decisions are compatibility-sensitive and easy to test with fixtures.

### 2. Keep parsing shells explicit

**Choice:** NBT traversal, byte readers, packet variants, storage mutation, and render invalidation stay in shells that convert raw inputs to core summaries and apply returned update plans.

**Rationale:** The pure core should not depend on packet reader state or rendering resources.

### 3. Preserve fallback behavior first

**Choice:** Existing unknown-dimension and malformed-data behavior remains stable unless a later protocol Cairn changes it.

**Rationale:** This extraction must not silently change client behavior.

### 4. Add fail-closed tests

**Choice:** Negative tests should cover missing selected dimension type, invalid min-y/height, truncated chunks, inconsistent section counts, unsupported biome payloads, and malformed light data.

**Rationale:** Most world regressions are bad-input or boundary bugs.

## Risks / Trade-offs

- Some packet parsing code may be macro/generated-adjacent; avoid broad rewrites while extracting decisions.
- Rendering invalidation is side-effect-heavy; leave it in shells until storage plans are stable.
- Protocol-version tests may need high-stack fixtures; keep them scoped to the smallest world core paths.
