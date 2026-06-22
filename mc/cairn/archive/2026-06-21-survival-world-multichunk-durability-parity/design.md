# Design: Survival world multichunk durability parity

## Context

Existing persistence evidence covers one ordinary block mutation under graceful restart and forced stop rows. This change defines a separate finite multi-chunk durability row.

## Decisions

### 1. Use explicit chunk coordinates

**Choice:** The row names each target block, chunk coordinate, mutation, restart/crash boundary, and post-restart observation.

**Rationale:** Multi-chunk persistence must be auditable without claiming arbitrary world durability.

### 2. Require source-of-truth storage evidence

**Choice:** Paper/reference and Valence fixtures must distinguish persisted world/storage state from auxiliary marker reconstruction.

**Rationale:** Reviewers need confidence that post-restart observations come from the configured storage path.

### 3. Keep broad durability separate

**Choice:** Long-term durability, backups, arbitrary crash consistency, and concurrent save races remain non-claims.

**Rationale:** Those require dedicated stress or fault-injection designs.
