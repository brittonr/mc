# Design: Bounded latency and jitter tolerance compatibility rail

## Context

This package comes from the 2026-05-25 ROI ranking after protocol-763 Valence CTF evidence had landed through scoring, BLUE/RED soaks, inventory/drop/pickup/block-place/click/open-container, and two-client combat/damage. The goal is to make the next compatibility claims receipt-backed without repeating saturated evidence.

## Decisions

### 1. Use bounded local perturbation only

**Choice:** Do not imply WAN, public-server, or adversarial network safety.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 2. Attach to an existing semantic rail

**Choice:** Reuse proven milestones so failures point to timing tolerance instead of new gameplay logic.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 3. Record perturbation parameters

**Choice:** Receipt must include delay/jitter/loss settings or an explicit unavailable-local-mechanism failure.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

## Risks / Trade-offs

- Linux traffic control or namespace permissions may be unavailable in the agent environment.
- Timing failures can be flaky unless perturbation bounds and timeouts are conservative.
