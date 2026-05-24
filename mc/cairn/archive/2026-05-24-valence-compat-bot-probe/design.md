# Design: Valence compat bot probe

## Context

Hyperion keeps `tools/rust-mc-bot` as a bot/load probe. Valence has `examples/bench_players.rs` and a `parkour-smoke` receipt, but no comparable real client-side oracle.

## Decisions

### 1. Keep this as a bounded owned probe

**Choice:** The port MUST be an internal compatibility tool with safe defaults, not a general-purpose public stress/DoS tool.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 2. Start with semantic receipt milestones

**Choice:** The first slice SHOULD prove status/login/render or example-specific milestones before expanding to multi-client/load.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 3. Integrate with existing receipts

**Choice:** The output SHOULD compose with `mc.compat.scenario.receipt.v2` style evidence rather than creating another ad hoc log artifact.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

## Risks / Trade-offs

- Hyperion bot code may not be reusable wholesale because target versions and protocol abstractions differ.
- A live bot gate can be flaky; deterministic dry-run gates must guard receipt contracts first.
