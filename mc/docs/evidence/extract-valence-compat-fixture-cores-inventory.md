# Valence compatibility fixture core extraction inventory

## Question

What fixture logic in `servers/valence/examples/ctf.rs` and `servers/valence/examples/survival_compat.rs` must move behind deterministic fixture-core boundaries for Cairn change `extract-valence-compat-fixture-cores`?

## Requirement IDs and task dependencies

- r[mc_compatibility.valence_fixture_core.inventory] inventories responsibilities before extraction.
- r[mc_compatibility.valence_fixture_core.boundaries] separates pure core decisions from Bevy adapter systems.
- r[mc_compatibility.valence_fixture_core.ctf_core] covers CTF flag, score, race, spawn-reset, inventory, combat, and milestone decisions.
- r[mc_compatibility.valence_fixture_core.survival_core] covers survival block/place, containers, crafting, furnace, hunger, mob, redstone, persistence, block-entity, biome/dimension, and milestone decisions.
- r[mc_compatibility.valence_fixture_core.state_ownership] removes or documents global mutable fixture state.
- r[mc_compatibility.valence_fixture_core.compatibility] preserves commands, env vars, milestones, behavior, and non-claims.
- r[mc_compatibility.valence_fixture_core.tests] requires positive and negative fixture-core tests.
- r[mc_compatibility.valence_fixture_core.validation] requires focused Valence checks, selected mc-compat rails, Cairn gates, validation, and task-evidence checks.

Dependency chain: inventory -> boundaries -> CTF/survival cores -> state ownership -> compatibility -> tests -> validation.

Owner subtree: `servers/valence/examples/` plus review evidence under `docs/evidence/`. No Hyperion code or concepts were adopted, ported, or referenced for this change.

## CTF fixture inventory

| Area | Rule decisions now in pure core | Adapter-owned shell responsibilities | Stable milestone/env contract | Non-goals |
| --- | --- | --- | --- | --- |
| Flag ownership and invalid pickups | Enemy flag accept, own-flag rejection, duplicate held-flag rejection, flag presence labels. | Read `DiggingEvent`, query `ChunkLayer`, mutate `FlagManager`, inventory, and block state. | `MC_COMPAT_CTF_INVALID_*`, `MC-COMPAT-MILESTONE invalid_flag_*`. | Does not prove full CTF correctness or vanilla flag rules. |
| Score limit and race | Race accept/reject/final-state validation, duplicate pickup guard, score snapshots, duplicate-win names. | Consume score resources, update `WinConditionState`, emit scoreboard/resource side effects. | `MC_COMPAT_CTF_SCORE_LIMIT_PROBE`, `MC_COMPAT_CTF_RACE_PROBE`, existing score/race milestone strings. | No broad minigame production readiness claim. |
| Spawn/team reset | Assignment deferral rule, balanced assignment state, reset preconditions. | Apply spawn positions, team layers, inventory loadouts, resource updates. | `MC_COMPAT_CTF_SPAWN_TEAM_RESET_PROBE`, spawn/resource reset milestones. | No full team balancer claim. |
| Inventory probes | Stack split/merge and drag transaction classifiers over explicit click snapshots. | Convert Valence `ClickSlotEvent`/`SlotChange`/`ItemStack`, open windows, mutate probe state, print milestones. | `MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE`, `MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE`, existing inventory milestones. | No claim for all inventory transaction families. |
| Combat/projectile policy | Reference hit matching, armor mitigation, knockback metric, arrow policy validation/evaluation outputs. | Read env/file policy config, reload requests, health mutation, velocity packets, status triggers, logging. | `MC_COMPAT_VANILLA_COMBAT_*`, `MC_COMPAT_PROJECTILE_PROBE`, `MC_COMPAT_STEEL_*`, existing combat/projectile/Steel milestones. | No vanilla combat parity beyond existing bounded rows. |

## Survival fixture inventory

| Area | Rule decisions now in pure core | Adapter-owned shell responsibilities | Stable milestone/env contract | Non-goals |
| --- | --- | --- | --- | --- |
| Block break/place | Survival mode, hand/state, target position, and face predicates. | Mutate `ChunkLayer`, inventory slots, pickup animation, packets. | Base survival block/place milestones. | No full survival block semantics. |
| Containers/crafting/furnace | Open predicates, slot/window/stack matching, collect events, invalid fuel rejection. | Spawn/open inventories, cursor items, reconnect reopen logging. | `MC_COMPAT_SURVIVAL_CHEST_FIXTURE`, `MC_COMPAT_SURVIVAL_CRAFTING_*`, `MC_COMPAT_SURVIVAL_FURNACE_*`. | No full recipe or container implementation claim. |
| Hunger and mob drop | Hunger profile selection, consume preconditions, mob attack/drop stack predicates. | Update `Health`, `Food`, `Saturation`, spawn item entities and pickup packets. | `MC_COMPAT_SURVIVAL_HUNGER_*`, `MC_COMPAT_SURVIVAL_MOB_*`. | No mob AI or loot table completeness claim. |
| Redstone | Toggle input predicate and pure power transition. | Apply block states/properties and emit synthetic breadth milestones. | `MC_COMPAT_SURVIVAL_REDSTONE_*`. | No full redstone simulation claim. |
| Persistence and block entities | Marker decision, post-restart missing-marker rejection, sign payload validation. | Read/write marker files, create dirs, set sign NBT, send block update packets. | `MC_COMPAT_SURVIVAL_WORLD_PERSISTENCE_*`, `MC_COMPAT_SURVIVAL_BLOCK_ENTITY_*`. | No arbitrary NBT/block-entity persistence claim. |
| Biome/dimension breadth | Known environment normalization and unknown fail-closed derivation. | Read env/scenario state and print fixture milestones. | `MC_COMPAT_SURVIVAL_BIOME_DIMENSION_*`. | No full dimension travel or biome parity claim. |

## Boundary decision

- Pure cores live in `servers/valence/examples/fixture_core/{ctf.rs,survival.rs}` and accept explicit snapshots/enums/values. They perform no Bevy queries, command mutation, env reads, filesystem access, packet writes, logging, or printing.
- `ctf.rs` and `survival_compat.rs` remain Bevy adapters. They translate Valence events/resources/items into core inputs, apply returned decisions, mutate ECS/world state, read env vars, perform marker file I/O, and emit receipt-stable milestone text.
- The prior global arrow policy stores were replaced by the explicit Bevy resource `ArrowPolicyState`; policy file reads and env reload checks stay in the shell.
- Remaining broad behavior is a non-claim unless separate accepted aggregate gates promote it: no broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, or full vanilla parity.

## Verification evidence

- Preflight Cairn gates/validation: `docs/evidence/extract-valence-compat-fixture-cores-preflight-gates.run.log`.
- Baseline Valence example tests before core edits: `docs/evidence/extract-valence-compat-fixture-cores-baseline-valence-examples.run.log`.
- Post-extraction focused Valence example tests: `docs/evidence/extract-valence-compat-fixture-cores-post-core-valence-examples.run.log`.
- Formatting check: `docs/evidence/extract-valence-compat-fixture-cores-fmt-check.run.log`.
- Selected dry-run rails for maintained CTF and survival command shapes: `docs/evidence/extract-valence-compat-fixture-cores-selected-dry-runs.run.log`.
