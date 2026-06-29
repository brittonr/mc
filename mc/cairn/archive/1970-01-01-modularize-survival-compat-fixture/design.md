# Design: Survival compatibility fixture modules

## Context

The survival fixture covers many independent bounded scenarios. Some pure helper functions already exist near the bottom of the file, but the scenario families remain interleaved with Bevy system shells and marker-file side effects. The modularization should make each bounded scenario family independently reviewable.

## Decisions

### 1. Split by survival fixture family

**Choice:** Create focused modules for runtime config, base arena setup, break/place/pickup, chest, crafting, furnace, hunger/health, mob-drop, redstone, persistence, block-entity, breadth-only fixtures, biome/dimension, sign editing, and milestone formatting.

**Rationale:** Each scenario family has different invariants, inputs, and non-claims.

### 2. Extract pure predicates and transitions

**Choice:** Move item/slot classification, click-event classification, hunger/mob-drop/redstone decisions, persistence phase decisions, and milestone text builders into pure functions.

**Rationale:** These are the units that need focused positive and negative coverage without a running server.

### 3. Keep durable side effects in shells

**Choice:** Marker-file writes, Bevy ECS mutations, inventory/window changes, packet/event emission, and milestone logging remain in shell systems that call pure cores.

**Rationale:** The shell boundary keeps evidence-producing side effects explicit.

### 4. Preserve bounded evidence semantics

**Choice:** Module names and tests must keep survival fixture non-claims and bounded fixture identities visible.

**Rationale:** This refactor must not broaden dry-run or live fixture evidence.

## Risks / Trade-offs

- Many helper constants are shared across scenario families; prefer small typed fixture structs over a new shared global constant bag.
- Some extraction may require short-lived adapters around Bevy event/query types; remove adapters when pure core inputs are stable.
- Persistence marker side effects are review-critical; keep their paths and phase semantics unchanged.
