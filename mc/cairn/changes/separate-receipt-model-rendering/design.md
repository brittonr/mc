# Design: Separate mc-compat receipt model, rendering, and writing

## Context

Receipts are review-critical artifacts. Building them as JSON strings directly obscures which fields are data decisions, which are rendering details, and which are filesystem side effects.

## Decisions

### 1. Introduce typed receipt inputs

**Choice:** Define `ReceiptInput`-style structs containing config-derived fields, run result, evidence, child revisions, paths, and non-claim context.

**Rationale:** Inputs become explicit and can be fixture-tested without file writes.

### 2. Build typed receipt models before JSON

**Choice:** Construct typed scenario receipt models through pure functions, then render those models to deterministic JSON.

**Rationale:** Schema behavior can be tested structurally before string rendering.

### 3. Keep writer shell thin

**Choice:** The writer shell resolves paths, creates parent directories, writes files, and records artifact hashes; it does not decide receipt semantics.

**Rationale:** Receipt semantics stay deterministic and mock-free.

### 4. Preserve current schema by default

**Choice:** Field names, legacy compatibility blocks, non-claims, and selected/not-selected receipt sections remain unchanged unless explicitly scoped by a future schema Cairn.

**Rationale:** Review and downstream checks rely on stable receipt shapes.

## Risks / Trade-offs

- Typed models can be verbose; group fields by receipt section.
- Existing string-based tests may need transition helpers; keep both until model parity is proven.
- Manual JSON rendering must remain deterministic; preserve existing sorted/stable output expectations where applicable.
