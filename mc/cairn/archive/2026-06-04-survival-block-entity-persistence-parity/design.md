## Context

Survival persistence evidence currently has narrow paired rows for chest state, furnace state, graceful world restart of one ordinary block, and crash recovery of the same ordinary block. The current bundle still names all block entities as a non-claim. The next useful slice is therefore one configured block entity whose persisted payload is visible to both the server fixture and Stevenarella client after restart/reconnect.

## Decisions

### 1. Use one sign block entity as the bounded representative

**Choice:** The row covers one configured sign block entity at a dedicated survival fixture coordinate with exact text payload, restart/reconnect, client observation, and server-side persisted state evidence.

**Rationale:** A sign exercises a non-container block entity payload without overlapping the existing chest and furnace rows. It is reviewable as a deterministic text payload and avoids broad claims about every block-entity type, NBT shape, or UI/editor behavior.

### 2. Keep the comparator pure and row-specific

**Choice:** Extend `tools/check_survival_row_parity.rs` with a row contract for normalized `survival-block-entity-persistence-parity` metrics, and keep file I/O in the checker shell.

**Rationale:** The pure comparison core can reject Valence-only evidence, missing Paper evidence, stale child revisions, missing sign text, wrong position, wrong block entity kind, or mismatched post-restart payload without standing up a live backend.

### 3. Separate this row from broad durability and crash recovery

**Choice:** The runner scenario, scenario manifest, matrix row, and bundle copy must keep existing graceful world-persistence and crash-recovery rows unchanged.

**Rationale:** This row is only a restart/reconnect proof for one configured sign block entity. Long-term durability, arbitrary crash consistency, all block entities, NBT breadth, editing UI semantics, concurrent saves, backups, and production readiness remain explicit non-claims.

## Risks / Trade-offs

- Sign text packet/client rendering timing may need extra milestone slack, so the row should compare normalized server state and client observation rather than incidental packet order.
- Paper and Valence may expose sign text formatting differently; the contract should normalize the exact plain-text payload and reject decoration-only evidence.
- The Paper fixture must not synthesize the post-restart result from an auxiliary marker file; reviewable evidence must prove the configured sign payload is loaded from the persisted world state.
