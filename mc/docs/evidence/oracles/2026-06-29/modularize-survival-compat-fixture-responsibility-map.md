# Modularize survival compatibility fixture responsibility map

## Question

What responsibilities did `servers/valence/examples/survival_compat.rs` own before modularization, and what focused owner should reviewers inspect after the split?

## Inspected evidence

- `servers/valence/examples/survival_compat.rs` pre-edit structure read on 2026-06-29.
- `servers/valence/examples/fixture_core/survival.rs` pre-edit pure-core helpers and tests read on 2026-06-29.
- Baseline command log: `docs/evidence/run-logs/2026-06-29/modularize-survival-compat-fixture.baseline-valence-survival-example-test.run.log`.

## Decision

The change is a fixture-architecture refactor only. The shell remains responsible for Bevy ECS access, resource insertion/mutation, packet writes, marker-file writes, and `info!`/stdout milestone emission. Pure decisions and bounded milestone construction move behind focused `fixture_core::survival` modules so representative positive and negative tests can exercise them without a running server.

| Responsibility | Pre-edit owner | Post-split owner | Boundary |
| --- | --- | --- | --- |
| Runtime config flag/phase/path decisions | `survival_compat.rs` runtime config helpers | `fixture_core::survival::runtime_config` | Pure flag, phase, marker-path, and invalid-config diagnostics; env reads stay in shell inputs. |
| Arena break/place predicates | `survival_compat.rs` wrappers plus `fixture_core::survival.rs` | `fixture_core::survival::arena` | Pure target predicates; chunk/block mutation stays in setup/systems. |
| Container open/slot/stack classification | `survival_compat.rs` and monolithic core helpers | `fixture_core::survival::containers` | Pure open/click/stack decisions; inventory/window mutation stays in shell systems. |
| Crafting click/collect decisions | `survival_compat.rs` crafting helpers | `fixture_core::survival::crafting` | Pure slot and result collection decisions; inventory writes and logs stay in shell. |
| Furnace slot/output/invalid-fuel decisions | `survival_compat.rs` furnace helpers plus monolithic core | `fixture_core::survival::furnace` | Pure slot, output, and invalid-fuel predicates; furnace inventory mutation stays in shell. |
| Hunger/health profile and use decisions | `survival_compat.rs` profile wrappers plus monolithic core | `fixture_core::survival::hunger_health` | Pure profile selection and consume precondition; component mutation stays in shell. |
| Mob-drop attack and pickup transition | `survival_compat.rs` pickup planner plus monolithic core | `fixture_core::survival::mob_drops` | Pure attack and pickup-state transition; entity spawn/despawn and packet writes stay in shell. |
| Redstone transition | `survival_compat.rs` toggle helpers plus monolithic core | `fixture_core::survival::redstone` | Pure powered-state transition; chunk block updates stay in shell. |
| Persistence marker decision | `survival_compat.rs` persistence helpers plus monolithic core | `fixture_core::survival::persistence` | Pure marker decision; marker-file creation/writes stay in shell. |
| Block-entity sign decisions | `survival_compat.rs` block-entity helpers plus monolithic core | `fixture_core::survival::block_entities` | Pure sign placement and payload validation; NBT construction and marker writes stay in shell. |
| Biome/dimension identity | `survival_compat.rs` biome helpers plus monolithic core | `fixture_core::survival::biome_dimension` | Pure environment normalization; live environment observation stays in shell. |
| Breadth-only synthetic fixtures | `survival_compat.rs` log helper family | `fixture_core::survival::breadth` | Pure bounded milestone text; emission stays in shell. |
| Sign-editing breadth fixture | `survival_compat.rs` sign log helper | `fixture_core::survival::sign_editing` | Pure bounded milestone text; emission stays in shell. |
| Milestone vocabulary shared by cores | `survival_compat.rs` and monolithic core formatting | `fixture_core::survival::milestones` plus family modules | Pure string construction; logging side effects stay in shell. |

## Owner

Valence survival compatibility fixture owner.

## Next action

Run focused Valence example tests and mc-compat dry-runs after the split, then cite reviewable logs and a BLAKE3 manifest before checking tasks.
