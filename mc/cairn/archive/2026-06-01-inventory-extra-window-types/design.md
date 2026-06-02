# Design: Inventory extra window types rail

## Context

`extra inventory window types` is still not covered by current promoted evidence, or it is covered only by a historical/oracle workaround. This change creates a narrow lifecycle package so future implementation has an evidence contract before code or docs broaden the claim.

## Decisions

### 1. Contract first

**Choice:** Define scope as one additional configured window type with open, click/transfer, close, and final inventory/window state metrics.

**Rationale:** The row must be reviewable before receipts are promoted.

### 2. Normalize metrics

**Choice:** Compare window type, window id, opened title/type, slot mapping, action item/count, final window slot state, final player inventory state, and server correlation.

**Rationale:** Logs and pass/fail alone are too coarse for this claim.

### 3. Fail closed

**Choice:** Reject missing window type, wrong slot mapping, missing open/close, wrong final item state, Valence-only vanilla parity, or all-window overclaim.

**Rationale:** Missing evidence should preserve non-claims instead of silently becoming coverage.

### 4. Keep broad claims blocked

**Choice:** Preserve these non-claims: all window types, crafting/furnace/chest rows already scoped elsewhere, all container transactions, all inventory semantics, and production readiness.

**Rationale:** This row should reduce one gap only.

## Implementation notes

- Select next window type
- Define slot mapping contract
- Add checker fixtures
- Promote only selected window row

## Risks / Trade-offs

- Window slot mappings differ by type; checker must require an explicit mapping table.
- If the selected window implies vanilla parity, use paired Paper evidence.
