# Design: Stevenarella hotspot module reduction

## Context

Stevenarella has already gained focused modules for capture, MCP, resources, game shell, and parts of entity/player behavior. Secondary hotspots should follow the same pattern: a small façade with explicit child modules and pure cores where possible.

## Decisions

### 1. Inventory before splitting

**Choice:** For each hotspot, record current public items, internal responsibilities, tests, and consumers before moving code.

**Rationale:** These modules expose broad APIs; accidental path/API churn can break unrelated client code.

### 2. Use façade modules

**Choice:** Keep existing `mod.rs` files as small façades that declare/re-export child modules and retain stable public names where practical.

**Rationale:** This reduces review size while avoiding a broad API migration.

### 3. Separate pure decisions from adapters

**Choice:** Move parsing, normalization, layout decisions, ECS query planning, and state transitions into pure functions. Keep renderer, GL, filesystem, network, input, and global console interactions in shells.

**Rationale:** Pure logic can have positive and negative tests without standing up the full client.

### 4. Work one hotspot at a time

**Choice:** Drain hotspots in small waves, starting with the highest churn or largest review bottleneck.

**Rationale:** Smaller changes reduce merge risk and make validation scope clear.

## Risks / Trade-offs

- Public module paths may be relied on internally; use re-exports and focused tests to keep call sites stable.
- Some modules combine rendering and data decisions; avoid forcing pure cores where graphics API state is the actual domain.
- Moving UI or ECS code can create cyclic imports; split by data/dependency direction first.
