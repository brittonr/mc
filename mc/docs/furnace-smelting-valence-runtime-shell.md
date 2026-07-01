# Furnace smelting Valence runtime shell

## Scope

This document records the first selected-row Valence runtime shell for furnace smelting. The shell remains bounded to Java Edition 1.20.1 / protocol 763 vocabulary, standard furnace only, selected input `minecraft:raw_iron`, fuel `minecraft:coal`, and output `minecraft:iron_ingot`.

The implementation is an explicit opt-in Bevy/ECS plugin in `servers/valence/examples/survival_compat.rs` named `SurvivalFurnaceSmeltingPlugin`. It is not part of Valence `DefaultPlugins`.

## Implemented artifacts

- Pure core: `servers/valence/examples/fixture_core/survival/furnace.rs`.
- Shell plugin/resources/events/system: `servers/valence/examples/survival_compat.rs`.
- Shared gameplay contract helpers: `servers/valence/examples/gameplay_contracts/mod.rs`.

The fixture-local pure core mirrors the selected-row checker semantics from `tools/check_furnace_smelting_core.rs`: it accepts plain furnace state, recipe rows, fuel rows, and limits, then returns a new state plus `StartedFuel`, `AdvancedCooking`, `ProducedOutput`, or a safe pause/error diagnostic.

The shell snapshots the Valence furnace inventory slots, runtime counters, selected recipe/fuel resources, and block-entity status into the pure core. It commits only returned state for successful transitions and records typed diagnostics for pauses, malformed data, unsupported furnace kind, missing inventory, and stale/unloaded block-entity state.

## Opt-in plugin boundary

`SurvivalFurnaceSmeltingPlugin` owns:

- `SurvivalFurnaceRecipeTableResource`.
- `SurvivalFurnaceFuelTableResource`.
- `SurvivalFurnaceSmeltingConfigResource`.
- `SurvivalFurnaceBlockEntity`.
- `SurvivalFurnaceStateChangedEvent`.
- `SurvivalFurnaceDiagnosticEvent`.

The plugin registers an explicit gameplay contract with `GameplayInstallMode::ExplicitOptIn`, `GameplayScopeModel::ArenaOwnedLayer`, and `Update` schedule phases using the existing survival gameplay phase order. Disabled-plugin tests prove the shell-owned resources, events, contract, and mutation are absent when the plugin is not installed.

## Validation evidence

- Baseline fixture/core/receipt/contract checks: `docs/evidence/furnace-smelting-valence-runtime-shell-baseline-2026-07-01.run.log`.
- Focused positive and negative shell tests: `docs/evidence/furnace-smelting-valence-runtime-shell-focused-validation-2026-07-01.run.log`.
- Schedule hygiene: `docs/evidence/furnace-smelting-valence-runtime-shell-schedule-hygiene-2026-07-01.run.log`.
- Affected Valence example check: `docs/evidence/furnace-smelting-valence-runtime-shell-valence-example-2026-07-01.run.log`.
- Formatter check: `docs/evidence/furnace-smelting-valence-runtime-shell-rustfmt-2026-07-01.run.log`.
- Sync and Cairn gates: `docs/evidence/furnace-smelting-valence-runtime-shell-sync-execute-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-accepted-spec-verify-2026-07-01.run.log`, and `docs/evidence/furnace-smelting-valence-runtime-shell-cairn-gates-2026-07-01.run.log`.
- Final lifecycle checks: `docs/evidence/furnace-smelting-valence-runtime-shell-task-evidence-final-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-archive-task-evidence-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-evidence-manifest-final-check-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-archive-dry-run-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-archive-execute-2026-07-01.run.log`, and `docs/evidence/furnace-smelting-valence-runtime-shell-post-archive-cairn-validate-2026-07-01.run.log`.
- Evidence summary: `docs/evidence/furnace-smelting-valence-runtime-shell-2026-07-01.md`.

## Positive shell coverage

Focused tests cover selected-row fuel start, active burn progress without extra fuel consumption, output production, and compatible output merge.

## Negative shell coverage

Focused tests cover invalid input, no fuel, blocked output, unsupported furnace kind, malformed recipe data, stale/unloaded block entity state, and disabled-plugin behavior. Negative cases assert no false inventory mutation and no shell transition is emitted for rejected state.

## Non-claims

This runtime shell does not claim DefaultPlugins membership, all-recipe breadth, all-fuel breadth, smoker behavior, blast-furnace behavior, hopper automation, XP behavior, recipe-book synchronization, chunk-unload semantics, Paper parity rerun, broad furnace parity, broad vanilla parity, broad Minecraft compatibility, public-server safety, or production readiness.
