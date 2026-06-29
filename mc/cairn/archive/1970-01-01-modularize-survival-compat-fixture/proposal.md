# Proposal: Modularize Valence survival compatibility fixture

## Why

`servers/valence/examples/survival_compat.rs` combines runtime config, arena setup, chest, crafting, furnace, hunger, mob-drop, redstone, world persistence, block-entity, biome/dimension, sign-editing, breadth fixtures, milestone formatting, and tests in one large example file. The file is a compatibility evidence surface, so mixed ownership makes review and future fixture additions risky.

## What Changes

- Split survival fixture behavior into focused modules for runtime config, arena/setup, containers, crafting, furnace, hunger/health, mob drops, redstone, persistence, block entities, biome/dimension, sign editing, and milestone formatting.
- Extract pure fixture decision cores for predicates, state transitions, item/slot classification, milestone construction, and persistence phase decisions.
- Keep Bevy ECS access, resource mutation, packet/event emission, marker-file writes, and logging in thin shells.
- Preserve existing env flags, fixture semantics, milestone vocabulary, evidence boundaries, and non-claims.
- Add positive and negative tests for extracted survival fixture families.

## Impact

- **Files**: `servers/valence/examples/survival_compat.rs`, new survival fixture modules or `fixture_core` modules, focused tests, affected docs if boundaries are documented, and Cairn artifacts.
- **Testing**: baseline Valence survival/example checks, positive and negative fixture-core tests, affected mc-compat dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: fixture architecture only; this does not promote full survival correctness, public-server readiness, or broad Minecraft compatibility evidence.
