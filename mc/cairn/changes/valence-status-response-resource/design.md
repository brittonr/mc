# Design: Valence status response resource

## Context

Hyperion commit `313503c` made server ping response data a resource. In Valence, a similar seam should be additive and API-shaped for examples/plugins.

## Decisions

### 1. Make status data resource-owned

**Choice:** Server list ping response data should be configurable through a stable resource rather than hard-coded inside packet handling.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 2. Keep defaults compatible

**Choice:** Existing examples should keep their current default response unless they opt into configuration.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 3. Use this as a test oracle

**Choice:** Status-only probes should be able to assert exact configured fields for deterministic compatibility evidence.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

## Risks / Trade-offs

- Public API shape may need to match Valence style and avoid overfitting to our harness.
- Minecraft status response fields differ by protocol/version; tests should assert current supported semantics only.
