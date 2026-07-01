# Furnace smelting Valence runtime shell inventory

## Question

What exact selected-row prerequisites and Valence APIs bound the first opt-in furnace smelting runtime shell?

## Inspected evidence

Baseline prerequisite artifacts:

- `docs/furnace-smelting-behavior-card.md` records the selected standard-furnace row, target scope, and non-claims.
- `tools/check_furnace_smelting_core.rs` records the existing selected-row pure core and fixture handoff semantics.
- `compat/config/furnace-smelting-selected-row-fixture.ncl` records RawIron + Coal -> IronIngot timing and stack constants.
- `tools/check_furnace_smelting_receipt_handoff.rs` bridges the selected fixture to archived Paper/reference and Valence receipt rows.
- `docs/furnace-smelting-valence-shell-contract.md` records the required opt-in shell contract.

Valence APIs inspected:

- `servers/valence/examples/survival_compat.rs`: existing survival fixture plugin, `SurvivalFurnaceFixture`, furnace open/click handlers, fixture slot constants, survival gameplay phase wiring, and `DefaultPlugins` call site.
- `servers/valence/examples/fixture_core/survival/furnace.rs`: existing fixture-local furnace helpers extended with selected-row pure core types and transitions.
- `servers/valence/examples/gameplay_contracts/mod.rs`: explicit opt-in gameplay contract metadata, scope checks, schedule phase helpers, and disabled-contract test helpers.
- `servers/valence/crates/valence_inventory/src/lib.rs`: `InventoryPlugin`, inventory schedule sets, `InventoryMutationSet`, `InventoryPresentationSet`, and packet presentation boundary.
- `servers/valence/crates/valence_inventory/src/model/storage.rs`: `Inventory::slot`, `set_slot`, `replace_slot`, `kind`, and changed-slot behavior.
- `servers/valence/crates/valence_inventory/src/model/catalog.rs`: `InventoryKind::Furnace` three-slot inventory and non-selected `BlastFurnace`/`Smoker` inventory kinds.
- `servers/valence/src/lib.rs`: `DefaultPlugins` group membership; the new furnace shell remains outside it.
- `servers/valence/tools/dump_schedule/README.md`: schedule hygiene trigger and receipt guidance.

Baseline command evidence is in `docs/evidence/furnace-smelting-valence-runtime-shell-baseline-2026-07-01.run.log` with `overall_exit_status=0`.

## Decision

Implement a focused example-local `SurvivalFurnaceSmeltingPlugin` that is explicit opt-in, owns its recipe/fuel/config/block-entity resources and state/diagnostic events, runs in the survival `Update`/`WorldMutation` phase, snapshots inventory and counters into the pure selected-row core, and commits only returned state.

Keep Valence `DefaultPlugins` unchanged. Keep broad recipe/fuel extraction, smoker/blast-furnace behavior, hopper automation, XP, recipe-book synchronization, chunk unload behavior, live Paper reruns, and production claims out of scope.

## Owner

Cairn change `add-furnace-smelting-valence-runtime-shell`.

## Next action

Promote focused shell tests, schedule hygiene evidence, task-evidence validation, accepted-spec sync, and archive receipts before claiming the runtime shell is drained.
