# Design: Protocol-763 compatibility acceptance matrix and gap checklist

## Context

This package comes from the 2026-05-25 ROI ranking after protocol-763 Valence CTF evidence had landed through scoring, BLUE/RED soaks, inventory/drop/pickup/block-place/click/open-container, and two-client combat/damage. The goal is to make the next compatibility claims receipt-backed without repeating saturated evidence.

## Decisions

### 1. Make the matrix evidence-index-like

**Choice:** Rows should point to durable receipts/docs and BLAKE3s rather than restating prose claims.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 2. Separate claims from non-claims

**Choice:** The matrix must show what remains unproven, including full CTF, production load, broad protocol coverage, and unbounded soak.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 3. Keep the first slice docs/checker only

**Choice:** Avoid expanding gameplay semantics while building the index; it is a coordination artifact.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

## Risks / Trade-offs

- Manual matrix rows can stale unless the checker validates required fields.
- Overbroad wording could accidentally imply full compatibility.
