# Design: Flag-carrier death and flag-return compatibility rail

## Context

This package comes from the 2026-05-25 ROI ranking after protocol-763 Valence CTF evidence had landed through scoring, BLUE/RED soaks, inventory/drop/pickup/block-place/click/open-container, and two-client combat/damage. The goal is to make the next compatibility claims receipt-backed without repeating saturated evidence.

## Decisions

### 1. Keep the scenario CTF-state first

**Choice:** The proof must bind flag ownership/return/reset to combat/death, not merely repeat generic damage milestones.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 2. Use server and client correlation

**Choice:** Valence must log carrier/death/return correlation, while Stevenarella must observe flag pickup, attack/death/health, and post-return state.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

### 3. Fail closed on accidental capture

**Choice:** A pass receipt must reject score/capture milestones when the test is meant to prove death-before-capture return semantics.

**Rationale:** Keep the slice independently drainable, evidence-backed, and scoped to owned local Valence/Stevenarella compatibility testing.

## Risks / Trade-offs

- Timing may be sensitive because the attack must happen after flag pickup but before capture.
- Valence may require explicit instrumentation for flag return/reset because the current example mostly logs score/inventory/combat milestones.
