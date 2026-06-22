# Design: Survival container and block-entity breadth parity

## Context

Chest persistence and sign block-entity persistence are promoted as separate narrow rows. This change defines one finite breadth row for additional containers and non-sign block-entity payloads.

## Decisions

### 1. Use a finite storage matrix

**Choice:** The row names exact container kinds, positions, transfer action or rejection, item metadata fields, and block-entity payloads.

**Rationale:** Storage breadth is large; exact rows prevent accidental all-container or arbitrary-NBT claims.

### 2. Compare metadata explicitly

**Choice:** Records normalize item id/count plus configured metadata payload identity rather than free-form NBT dumps.

**Rationale:** Explicit fields are reviewable and avoid leaking broad arbitrary-NBT claims.

### 3. Keep sign editing separate

**Choice:** Sign editing UI remains a separate row even when sign/block-entity packet evidence exists.

**Rationale:** Editing UI behavior has a different client interaction and validation surface.
