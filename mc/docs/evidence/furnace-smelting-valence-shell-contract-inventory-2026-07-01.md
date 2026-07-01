# Furnace smelting Valence shell contract inventory

## Scope

This inventory records the pre-contract furnace selected-row baseline and the Valence shell-planning sources for `add-furnace-smelting-valence-shell-contract`.

## Baseline evidence

The focused baseline log is `docs/evidence/furnace-smelting-valence-shell-contract-baseline-2026-07-01.run.log` and records `overall_exit_status=0` for:

- `tools/check_furnace_smelting_data_fixture.rs --self-test`.
- `tools/check_furnace_smelting_data_fixture.rs --fixture compat/config/furnace-smelting-selected-row-fixture.ncl`.
- `tools/check_furnace_smelting_core.rs --self-test --fixture compat/config/furnace-smelting-selected-row-fixture.ncl`.
- `tools/check_furnace_smelting_receipt_handoff.rs --self-test`.
- `tools/check_furnace_smelting_receipt_handoff.rs --fixture compat/config/furnace-smelting-selected-row-fixture.ncl --paper docs/evidence/survival-furnace-smelting-breadth-paper-2026-06-21.kv --valence docs/evidence/survival-furnace-smelting-breadth-valence-2026-06-21.kv`.

The validated selected row remains Java Edition 1.20.1 / protocol 763, standard furnace, `minecraft:raw_iron` plus `minecraft:coal` to `minecraft:iron_ingot`, output count one, cook ticks 200, and coal burn ticks 1600.

## Predecessor artifacts inspected

- `docs/furnace-smelting-behavior-card.md` records the selected-row behavior card, pure-core shape, future shell boundary, and non-claims.
- `docs/furnace-smelting-selected-row-core.md` records the pure deterministic selected-row core and local fixture handoff.
- `docs/furnace-smelting-selected-row-data-fixture.md` records the typed Java Edition 1.20.1 / protocol 763 fixture contract.
- `docs/furnace-smelting-selected-row-receipt-handoff.md` records the deterministic bridge to archived Paper/reference and Valence selected-row receipt evidence.
- `cairn/specs/vanilla-composable-plugins/spec.md` contains the accepted furnace behavior-card, selected-row core, data-fixture, and receipt-handoff requirements.

## Valence shell-planning sources inspected

- `servers/valence/README.md`, `servers/valence/CONTRIBUTING.md`, `servers/valence/AGENTS.md`, and `servers/valence/.agent/napkin.md` record Valence workflow, plugin modularity, cargo/devshell expectations, formatting constraints, and local agent notes.
- `servers/valence/src/lib.rs` shows `DefaultPlugins` membership. It currently adds core server/event-loop/client/layer plugins and feature-gated optional plugins such as `InventoryPlugin`; the furnace shell contract must not add furnace behavior to `DefaultPlugins`.
- `servers/valence/crates/valence_server/src/event/loop.rs` defines `RunEventLoop`, `EventLoopPreUpdate`, `EventLoopUpdate`, `EventLoopPostUpdate`, and `EventLoopSet` phases. Furnace smelting is not packet decoding, so the contract should not place rule evaluation in typed adapter phases.
- `servers/valence/examples/gameplay_contracts/mod.rs` defines the shared gameplay contract vocabulary: `GameplayInstallMode::ExplicitOptIn`, `GameplayScopeModel`, `GameplayPhase::{Input, RuleEvaluation, WorldMutation, Presentation, Cleanup}`, and schedule metadata helpers.
- `servers/valence/crates/valence_inventory/src/lib.rs` defines `InventoryPlugin` sets including `InventoryMutationSet`, `InventoryWindowSyncSet`, `InventoryPresentationSet`, and `InventoryCleanupSet`. Future furnace commits that affect inventories must happen before client-visible inventory presentation and packet flushing.
- `servers/valence/crates/valence_server/src/tick_scheduler.rs` records an explicit opt-in scheduler plugin pattern that does not install behavior by default.
- `servers/valence/tools/dump_schedule/README.md`, `docs/evidence/add-schedule-hygiene-gates.inventory.md`, and `docs/evidence/add-event-loop-phase-system-sets.inventory.md` record when schedule evidence and disabled-plugin comparisons are required.
- `cairn/specs/valence-bevy-ecs/spec.md` contains relevant accepted requirements for schedule hygiene, gameplay plugin contracts, inventory sets, and default-membership preservation.

## Runtime API facts deferred to implementation-time inspection

The shell contract deliberately does not assume exact future Valence storage types for furnace block entities, chunk/block-entity lookup, inventory slot routing, or client-visible block update packets. Future runtime implementation must inspect the affected crates and promote those exact APIs with focused Valence tests before claiming Valence runtime behavior.

## Non-claims retained

This inventory and the following shell contract do not add a Bevy system, plugin registration, schedule wiring, data-pack loader, Paper rerun, Valence runtime integration, DefaultPlugins membership change, all-recipe breadth, broad furnace parity, broad vanilla parity, public-server safety, or production readiness claim.
