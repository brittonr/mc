# Design: Promote recipe-book client settings evidence

## Context

Command/recipe evidence currently uses raw command/recipe packet fixtures and a crafting-table live receipt. Recipe-book client settings are adjacent but distinct: they are client preferences/state, not recipe execution.

## Decisions

### 1. Use one deterministic settings transition

**Choice:** Configure one recipe-book state change with exact booleans/category values.

**Rationale:** It avoids broad recipe-book UI and all-category claims.

### 2. Require Valence receipt of the packet

**Choice:** Evidence must include server correlation that the configured settings were received and parsed.

**Rationale:** A client-side intent alone is not enough for packet promotion.

### 3. Keep crafting semantics separate

**Choice:** Do not use this row to claim crafting correctness or recipe discovery.

**Rationale:** Existing crafting rows have separate reference-parity evidence.

## Risks / Trade-offs

- Stevenarella may not have recipe-book UI support; a protocol-level bounded probe may be necessary.
- Valence may need fixture instrumentation for this normally low-level packet.
- Category/boolean naming must be normalized to avoid protocol-version ambiguity.
