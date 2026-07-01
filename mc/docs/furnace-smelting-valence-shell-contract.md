# Furnace smelting Valence shell contract

## Target scope

This document defines the selected-row Valence shell contract for the furnace-smelting chain. The target remains Java Edition 1.20.1 / protocol 763, standard furnace only, selected-row input `minecraft:raw_iron`, fuel `minecraft:coal`, and output `minecraft:iron_ingot`.

The contract plans a thin opt-in Bevy/ECS shell around the existing selected-row core. It adds no runtime behavior in this package. No Valence runtime integration is implemented, no Bevy system is registered, no schedule is changed, and no DefaultPlugins membership change is permitted.

## Inventory and prerequisite evidence

The pre-contract inventory is `docs/evidence/furnace-smelting-valence-shell-contract-inventory-2026-07-01.md`. It records the focused baseline log, predecessor docs, accepted requirements, and Valence sources inspected before this contract was written.

Predecessor furnace artifacts:

- `docs/furnace-smelting-behavior-card.md` for the selected-row behavior card and first shell boundary sketch.
- `docs/furnace-smelting-selected-row-core.md` for the pure in-memory selected-row core.
- `docs/furnace-smelting-selected-row-data-fixture.md` for the Java Edition 1.20.1 / protocol 763 selected fixture.
- `docs/furnace-smelting-selected-row-receipt-handoff.md` for the archived Paper/reference plus Valence receipt bridge.
- `r[vanilla_composable_plugins.furnace_smelting_card]`, `r[vanilla_composable_plugins.furnace_smelting_core]`, `r[vanilla_composable_plugins.furnace_smelting_data_fixture]`, and `r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts]` for the accepted prerequisite chain.

Relevant Valence contract sources:

- `r[valence_bevy_ecs.schedule_hygiene.policy]` and `r[valence_bevy_ecs.schedule_hygiene.receipts]` for schedule evidence triggers and receipt shape.
- `r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract]` and `r[valence_bevy_ecs.gameplay_plugin_contracts.metadata]` for shared gameplay plugin metadata.
- `r[valence_bevy_ecs.inventory_sets.contract]` and `r[valence_bevy_ecs.inventory_sets.wiring]` for inventory mutation and presentation ordering.
- `servers/valence/src/lib.rs`, `servers/valence/crates/valence_server/src/event/loop.rs`, `servers/valence/examples/gameplay_contracts/mod.rs`, `servers/valence/crates/valence_inventory/src/lib.rs`, `servers/valence/crates/valence_server/src/tick_scheduler.rs`, and `servers/valence/tools/dump_schedule/README.md`.

## Core and shell boundary

future ECS systems snapshot furnace block-entity state, slot contents, recipe rows, fuel rows, and limits into explicit core inputs. They then call the selected-row core and apply only the returned state, transition, or typed error.

The pure core remains the only owner of selected-row rule decisions. The pure core must not read files, inspect environment variables, mutate Bevy world state, write packets, log, fetch network pages, or depend on wall-clock time.

The shell owns Bevy queries, resources, commands, schedule registration, inventory component access, block-entity component access, client-visible update events, packet-writing adapters, and diagnostics routing.

## Core input mapping

| Shell observation | Core input | Boundary rule |
| --- | --- | --- |
| Furnace block-entity kind | `FurnaceKind` | Only `standard` may reach the selected-row core until separate evidence broadens scope. |
| Input, fuel, and output slots | `FurnaceState` stack fields | The shell snapshots slots before rule evaluation and does not let the core borrow ECS state. |
| Cook and burn counters | `FurnaceState` tick fields | Tick counters are explicit values supplied by the shell. |
| Selected recipe table | `RecipeRow` values | Data loading occurs before the tick system and outside the core. |
| Selected fuel table | `FuelRow` values | Fuel rows are in-memory data resources before rule evaluation starts. |
| Stack and timing limits | `FurnaceLimits` and named constants | Constants must come from target-version data or reviewed fixture inputs. |

## Core output mapping

| Core output | Shell action | Boundary rule |
| --- | --- | --- |
| New `FurnaceState` | Commit back to the furnace component and inventory slots | The shell performs a single commit step after rule evaluation. |
| `FurnaceTransition` values such as `StartedFuel`, `AdvancedCooking`, or `ProducedOutput` | Emit `FurnaceStateChangedEvent` and mark affected slots/components changed | The event mirrors the returned transition and does not invent rule semantics. |
| `PausedNoFuel`, `PausedNoRecipe`, or `PausedOutputBlocked` | Preserve unchanged state and optionally emit `FurnaceDiagnosticEvent` | Diagnostics are derived from returned transitions. |
| Typed error such as malformed data or unsupported furnace kind | Reject the tick for that furnace and emit a typed diagnostic | The shell does not recover by applying partial mutations. |

## Opt-in plugin ownership

A future `FurnaceSmeltingPlugin` must be explicit opt-in. Its metadata should follow `GameplayInstallMode::ExplicitOptIn` and a scoped gameplay model such as `GameplayScopeModel::ArenaOwnedLayer` when it is attached to a survival arena or layer.

Planned shell-owned resources:

- `FurnaceSmeltingPluginContract`.
- `FurnaceRecipeTableResource`.
- `FurnaceFuelTableResource`.
- `FurnaceSmeltingConfigResource`.

Planned shell-owned components or adapters:

- `FurnaceBlockEntity` for the furnace runtime state owned by a block entity or layer scope.
- `FurnaceSlotSnapshot` as an implementation boundary for the input, fuel, and output slots.
- `FurnaceSmeltingProgress` if counters are split from the block-entity storage.

Planned shell-owned events:

- `FurnaceStateChangedEvent` for accepted returned transitions.
- `FurnaceDiagnosticEvent` for typed errors and paused transitions that reviewers decide should be observable.
- `FurnaceInventoryCommitEvent` if inventory mutation is routed through a separate commit phase.

The shell may read or mutate Valence `Inventory` state only through documented inventory APIs. It does not own the global inventory plugin and must not claim all inventory behavior.

## Schedule contract

Candidate schedule placement is `Update`, using the shared gameplay phase vocabulary:

- `GameplayPhase::Input` collects eligible furnace snapshots after the relevant event-loop input and data-loading boundaries.
- `GameplayPhase::RuleEvaluation` calls the pure selected-row core over snapshots.
- `GameplayPhase::WorldMutation` commits the returned furnace state and inventory slot changes.
- `GameplayPhase::Presentation` remains a downstream/client-visible boundary and should be left to existing inventory or packet presentation systems.
- `GameplayPhase::Cleanup` is only added if future implementation owns stale furnace cleanup.

Ordering dependencies:

- Data-loading or source-adapter systems must run before `GameplayPhase::RuleEvaluation`.
- Furnace slot commits must run after any same-tick `InventoryMutationSet` input mutation that can affect the same slots.
- Furnace commits must run before `InventoryPresentationSet` and before `FlushPacketsSet` so client-visible inventory packets observe committed state.
- The future implementation must record focused schedule hygiene evidence and a disabled-plugin comparison if it adds plugin wiring, schedule labels, system sets, ordering constraints, event-loop phases, or default plugin membership.

This contract does not choose a final schedule API for block-entity ticking. If source inspection shows a safer Valence-owned block-entity phase, the implementation may revise the candidate phase with evidence before runtime claims are promoted.

## Disabled-plugin behavior

When FurnaceSmeltingPlugin is not installed:

- no furnace resources are inserted;
- no furnace events are registered;
- no furnace systems run;
- no furnace slot, block-entity, inventory, packet, or layer mutation occurs;
- no furnace packets or milestones are emitted;
- no gameplay contract metadata is recorded for the furnace plugin;
- the pure core remains callable by tests because it is independent of Bevy and plugin installation.

A future Valence test must include a negative disabled-plugin case proving the shell-owned resources, events, systems, and metadata are absent when the plugin is not added.

## Data loading boundary

Recipe and fuel data enter through a startup shell or source adapter before furnace ticking. For this selected-row contract, the existing Nickel/exported fixture may seed tests, but runtime data-pack parsing is not implemented.

The furnace tick system must not parse data packs, evaluate Nickel, read JSON, read files, inspect process environment, or fetch network pages. It reads only typed in-memory `FurnaceRecipeTableResource` and `FurnaceFuelTableResource` values.

## Mutation, packet, and logging boundaries

The future shell uses a snapshot before rule evaluation and a single commit step after the core returns. Inventory or block-entity mutation is skipped when the core returns a typed error.

Packet writes remain outside the core. Client-visible slot, screen, or layer updates must be produced by existing Valence presentation boundaries or by explicit shell events that run after mutation and before packet flush. Logging remains outside the core; rule diagnostics travel as returned transitions or typed errors.

## Validation contract

Focused contract validation must include positive validation for this complete contract and negative validation for:

- missing target scope;
- missing selected-row prerequisites;
- missing core/shell boundary;
- missing shell ownership;
- missing schedule facts;
- missing disabled-plugin behavior;
- missing data-loading boundaries;
- missing mutation, packet, or logging boundaries;
- missing positive validation;
- missing negative validation;
- missing evidence requirements;
- missing non-claims;
- DefaultPlugins membership overclaims;
- broad recipe/furnace parity overclaims;
- broad vanilla parity, public-server safety, or production readiness overclaims.

## Future closeout prerequisites

before any Valence runtime behavior claim is promoted, follow-on implementation must record:

- exact Valence block-entity, inventory, and layer APIs inspected;
- positive runtime tests for selected-row fuel start, active burn, output production, and compatible output merge;
- negative runtime tests for invalid input, no fuel, blocked output, unsupported furnace kind, malformed data, stale or unloaded block entity, and disabled plugin;
- focused schedule evidence for any plugin wiring or ordering constraints;
- accepted-spec sync verification;
- task-evidence validation;
- evidence-manifest checks.

## Non-claims

This contract does not claim:

- No broad Minecraft compatibility.
- No broad vanilla parity.
- No full survival correctness.
- No all-recipe breadth.
- No all-fuel breadth.
- No broad furnace parity.
- No smoker behavior.
- No blast-furnace behavior.
- No hopper automation.
- No XP behavior.
- No recipe-book synchronization.
- No chunk-unload semantics.
- No Valence runtime integration.
- No DefaultPlugins membership change.
- No public-server safety.
- No production readiness.
- No data-pack loader.
- No Paper rerun.
