# Design: Valence gameplay oracles from Hyperion

## Context

Our existing Stevenarella evidence already exercises many CTF milestones. Hyperion gives a useful vocabulary for stronger gameplay oracles, but the implementation should be Valence-native.

## Decisions

### 1. Translate milestones, do not copy gameplay

**Choice:** The useful artifact is the semantic oracle vocabulary, not Hyperion Bedwars game code.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 2. Require client and server evidence for semantic claims

**Choice:** Non-smoke gameplay scenarios should require both client milestone evidence and Valence server correlation.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 3. Keep overclaim boundaries explicit

**Choice:** Each receipt should state which gameplay properties remain unproven.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

## Risks / Trade-offs

- Server logs may need explicit markers to avoid brittle substring matching.
- Adding too many gameplay scenarios at once can create a slow/flaky harness; slice by highest-value milestone first.
