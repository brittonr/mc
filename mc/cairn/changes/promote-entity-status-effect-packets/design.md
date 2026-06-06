# Design: Promote entity status-effect packet evidence

## Context

Potion/status-effect packet rows are visible gaps and adjacent to combat/survival systems. One deterministic apply/remove effect can cover the packet seam without claiming broad effect mechanics.

## Decisions

### 1. Use one harmless effect

**Choice:** Configure one deterministic effect with bounded duration and no broad gameplay assertion.

**Rationale:** Effects have many semantics; the row should prove packet observation and correlation only.

### 2. Include remove evidence when feasible

**Choice:** Promote apply and remove together only if both are observed; otherwise promote apply only and keep removal as a non-claim.

**Rationale:** Remove semantics require a separate packet and should not be inferred.

### 3. Avoid combat modifier claims

**Choice:** Do not claim damage/knockback/stat modifier behavior from this packet row.

**Rationale:** Modifier behavior belongs in separate combat or survival parity rows.

## Risks / Trade-offs

- Effect identifiers may differ by protocol mapping and need clear normalized names.
- Timed removal can be flaky; explicit fixture-triggered removal may be needed.
- Client rendering of particles/UI should remain out of scope unless separately observed.
