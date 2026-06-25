# Design: Harden Stevenarella runtime boundaries

## Context

Some unsafe code is expected in a renderer and compact ECS, but current boundaries make implicit assumptions global: GL context initialization order, resource manager sharing, and ECS component lifetimes. The change should narrow unsafe areas and increase assertion density without trying to rewrite the whole client at once.

## Decisions

### 1. Audit before replacing

Document each unsafe/global boundary, its caller assumptions, failure mode, and minimal safe wrapper. Do not refactor a boundary without first naming the invariant it relies on.

### 2. Make GL context explicit

Prefer an explicit `GlContext`/render context handle passed through rendering code. If a global remains temporarily, it should be initialization-checked, non-null by construction, and isolated to one small module.

### 3. Split resource sharing from resource IO

Separate immutable resource access, mutable pack reload/progress state, and cross-thread progress reporting. Remove unsafe `Sync` only after a safe ownership model or synchronization boundary is proven.

### 4. Quarantine ECS unsafe storage

Keep raw storage internals behind APIs that enforce generation, component membership, aliasing, and drop invariants. Pure tests should exercise invalid entity, stale generation, duplicate mutable borrow attempts where representable, and component removal/drop cases.

## Risks / Trade-offs

- Full ECS replacement is too large; start by documenting and testing invariants around existing storage.
- Passing GL context handles can touch many call sites; migrate renderer-owned paths first.
- This work reduces internal risk but does not claim complete memory-safety proof.
