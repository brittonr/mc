# Crafting recipe Valence shell contract inventory — 2026-07-05

## Scope

This inventory records the pre-contract crafting selected-matrix baseline and the Valence shell-planning sources for `add-crafting-recipe-valence-shell-contract`.

## Baseline evidence

The focused baseline log will be recorded in the task evidence. Self-test results from 2026-07-05:

- `check_crafting_recipe_behavior_card.rs --self-test`: passed.
- `check_crafting_recipe_core.rs --self-test`: passed.
- `check_crafting_recipe_data_fixture.rs --self-test`: passed.
- `check_crafting_recipe_receipt_handoff.rs --self-test`: passed.

The validated selected matrix remains Java Edition 1.20.1 / protocol 763:

- Shaped `minecraft:chest` from eight `minecraft:oak_planks` inputs in slots `1,2,3,4,6,7,8,9`, output `minecraft:chest` count `1`, primary-click collection into inventory slot `36`.
- Shapeless `minecraft:oak_planks` from one `minecraft:oak_log` input in slot `1`, output `minecraft:oak_planks` count `4`, primary-click collection into inventory slot `37`.
- Invalid `minecraft:stick_insufficient_input_rejection` with one `minecraft:oak_planks` input in slot `1`, empty result, `no_result` diagnostic.
- Primary-click collection only.

## Predecessor artifacts inspected

- `docs/crafting-recipe-behavior-card.md` records the accepted behavior card, selected matrix, pure-core shape, future shell boundary, and non-claims.
- `docs/crafting-recipe-selected-matrix-core.md` records the pure deterministic selected-matrix core and local fixture handoff.
- `docs/crafting-recipe-selected-matrix-data-fixture.md` records the typed Java Edition 1.20.1 / protocol 763 fixture contract and validated fixture rows.
- `docs/crafting-recipe-selected-matrix-receipt-handoff.md` records the deterministic bridge to archived Paper/reference and Valence selected-matrix receipt evidence.
- `cairn/specs/vanilla-composable-plugins/spec.md` contains the accepted crafting behavior-card, selected-matrix core, data-fixture, and receipt-handoff requirements (r[vanilla_composable_plugins.crafting_recipe_card], r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core], r[vanilla_composable_plugins.crafting_recipe_data_fixture], r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts]).

## Valence shell-planning sources inspected

- `servers/valence/README.md`, `servers/valence/CONTRIBUTING.md`, `servers/valence/AGENTS.md`, and `servers/valence/.agent/napkin.md` record Valence workflow, plugin modularity, cargo/devshell expectations, formatting constraints, and local agent notes.
- `servers/valence/src/lib.rs` shows `DefaultPlugins` membership. It currently adds core server/event-loop/client/layer plugins and feature-gated optional plugins such as `InventoryPlugin`; the crafting shell contract must not add crafting behavior to `DefaultPlugins`.
- `servers/valence/crates/valence_server/src/event/loop.rs` defines `RunEventLoop`, `EventLoopPreUpdate`, `EventLoopUpdate`, `EventLoopPostUpdate`, and `EventLoopSet` phases. Crafting is inventory-driven and must be placed relative to existing inventory sets.
- `servers/valence/examples/gameplay_contracts/mod.rs` defines the shared gameplay contract vocabulary: `GameplayInstallMode::ExplicitOptIn`, `GameplayScopeModel`, `GameplayPhase::{Input, RuleEvaluation, WorldMutation, Presentation, Cleanup}`, and schedule metadata helpers.
- `servers/valence/crates/valence_inventory/src/lib.rs` defines `InventoryPlugin` sets including `InventoryMutationSet`, `InventoryWindowSyncSet`, `InventoryPresentationSet`, and `InventoryCleanupSet`. Future crafting commits that affect inventories must happen before client-visible inventory presentation and packet flushing.
- `servers/valence/crates/valence_server/src/tick_scheduler.rs` records an explicit opt-in scheduler plugin pattern that does not install behavior by default.
- `servers/valence/tools/dump_schedule/README.md`, `docs/evidence/add-schedule-hygiene-gates.inventory.md`, and `docs/evidence/add-event-loop-phase-system-sets.inventory.md` record when schedule evidence and disabled-plugin comparisons are required.
- `cairn/specs/valence-bevy-ecs/spec.md` contains relevant accepted requirements for schedule hygiene (r[valence_bevy_ecs.schedule_hygiene.policy]), gameplay plugin contracts (r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract]), and inventory sets (r[valence_bevy_ecs.inventory_sets.contract]).
- `cairn/specs/vanilla-composable-plugins/spec.md` contains the accepted furnace smelting Valence shell contract requirements (r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract]) as analog precedent for crafting.

## Runtime API facts deferred to implementation-time inspection

The shell contract deliberately does not assume exact future Valence storage types for crafting table block entities, chunk/block-entity lookup, inventory slot routing, crafting grid state storage, client-visible block/container update packets, or click packet decoding. Future runtime implementation must inspect the affected crates and promote those exact APIs with focused Valence tests before claiming Valence runtime behavior.

## Non-claims retained

This inventory and the following shell contract do not add a Bevy system, plugin registration, schedule wiring, data-pack loader, Paper rerun, Valence runtime integration, DefaultPlugins membership change, all-recipe breadth, arbitrary collection modes, shift-click/drag/split breadth, recipe-book behavior, data-pack loading, automated crafter behavior, broad vanilla parity, public-server safety, or production readiness claim.