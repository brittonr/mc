# Design: Add a Valence packet compose API

## Context

Valence game logic can write packets through client components. Hyperion wraps outbound packet creation and routing in a composition API that can target one client, all clients, nearby clients, or channels. A Valence version should fit existing ECS and networking abstractions rather than copying Hyperion's exact API.

## Decisions

### 1. Route intents, not immediate side effects

**Choice:** Systems build packet bundles plus route intents. A later shell resolves the intent against current clients or proxy state and performs writes.

**Rationale:** The build/plan step can be tested without live clients and can feed either direct or proxy backends.

### 2. Direct backend first

**Choice:** Implement direct-mode delivery before proxy-specific wiring.

**Rationale:** This provides immediate Valence value and creates a stable contract for the proxy backend Cairn.

### 3. Preserve packet ordering within a bundle

**Choice:** Packet bundles preserve author order for each recipient unless a backend explicitly documents stronger batching constraints.

**Rationale:** Many Minecraft packet sequences are order-sensitive.

### 4. Report partial flush failures

**Choice:** Closed clients, invalid route targets, encode failures, and backend write errors must be reported as structured diagnostics.

**Rationale:** Silent packet drops make compatibility evidence hard to trust.

## Risks / Trade-offs

- A high-level API can hide packet costs; mitigate with docs and explicit bundle types.
- Route resolution can duplicate proxy work; mitigate by sharing a pure route planner where possible.
- Ordering guarantees can reduce backend optimization freedom; mitigate by documenting bundle-level ordering only.
