# Design: Stevenarella capture service split

## Context

Capture is both a user-visible artifact surface and an MCP/evidence support surface. It has good pure functions but module ownership is still broad. The split should retain existing public types where practical while moving implementation details into smaller modules.

## Decisions

### 1. Keep request and metadata models stable

**Choice:** Preserve public capture request, plan, artifact metadata, digest, and recording types unless a later schema change explicitly replaces them.

**Rationale:** MCP and evidence code rely on these shapes.

### 2. Split by side-effect class

**Choice:** Separate validation/planning, queueing, readback normalization, persistence, recording, and service orchestration.

**Rationale:** Each area has different side effects and test needs.

### 3. Push side effects to adapters

**Choice:** Filesystem writes, PNG encoding, framebuffer reads, clocks, and channels remain behind shell functions or adapters. Pure modules compute paths, dimensions, recording due-ness, buffer lengths, and metadata checks.

**Rationale:** Capture behavior becomes testable without OpenGL, filesystem, or real time.

### 4. Preserve MCP semantics

**Choice:** MCP capture output modes, resource reads, inline/artifact behavior, redaction state, and BLAKE3 metadata remain stable.

**Rationale:** Capture modularity must not break controlled evidence workflows.

## Risks / Trade-offs

- Image encoding is deterministic but still a side-effect-adjacent dependency; keep encoding boundary explicit.
- Queue tests should avoid flaky timing by using deterministic receiver state fixtures.
- Artifact path containment is safety relevant; preserve current fail-closed diagnostics.
