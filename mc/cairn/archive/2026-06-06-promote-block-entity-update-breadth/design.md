# Design: Promote one additional block-entity update breadth row

## Context

Sign block-entity evidence exercises one important `BlockEntityUpdateS2CPacket` shape, but it leaves arbitrary block-entity and NBT breadth as non-claims. One additional non-sign fixture can improve confidence without broadening beyond reviewable bounds.

## Decisions

### 1. Use one non-sign block entity

**Choice:** Select one deterministic non-sign kind, preferably chest or skull, with a small normalized payload.

**Rationale:** A second kind demonstrates the packet path beyond sign text while remaining testable.

### 2. Compare normalized payload fields only

**Choice:** Validate kind, position, and a small payload metric, not full NBT equivalence.

**Rationale:** Arbitrary NBT parity is too broad for this row.

### 3. Keep persistence separate unless explicitly tested

**Choice:** This row covers update observation, not restart persistence, unless the contract adds paired persistence evidence.

**Rationale:** Persistence is already covered by separate rows and has different failure modes.

## Risks / Trade-offs

- Stevenarella may not render or expose the chosen block entity; choose a payload with observable logs or parser events.
- Paper/reference setup may need a fixture plugin for deterministic payloads.
- Avoid implying all block-entity update shapes or arbitrary NBT support.
