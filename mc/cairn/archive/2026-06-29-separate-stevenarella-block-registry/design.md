# Design: Stevenarella block registry separation

## Context

The block crate uses macros to define a large set of block facts. The runtime API also lives near that data. A separation should make generated data, public runtime types, id maps, and helper behavior independently reviewable.

## Decisions

### 1. Split generated data from runtime API

**Choice:** Move generated or declarative block definitions into a dedicated module and keep hand-authored APIs in runtime modules.

**Rationale:** Reviewers can distinguish data churn from logic changes.

### 2. Preserve public exports through adapters

**Choice:** Keep existing public names and imports available from the crate root while moving implementation behind modules.

**Rationale:** Stevenarella call sites should not require broad migration during the split.

### 3. Make data freshness explicit

**Choice:** If a generator or generated snapshot exists, add a deterministic check that proves checked-in block data matches the source.

**Rationale:** Generated registry changes should be intentional and reviewable.

### 4. Keep fallback semantics stable

**Choice:** `VanillaIDMap` lookup behavior, missing-block fallback, modded pre-1.13 paths, material and collision semantics remain stable.

**Rationale:** This is not a block-compatibility expansion.

## Risks / Trade-offs

- Macro expansion may make small moves noisy; extract module boundaries before changing data shape.
- Public `pub use Block::*` may need compatibility re-exports; preserve it until a separate API-change Cairn removes it.
- Generated data tests may need fixture snapshots to avoid brittle full-registry assertions.
