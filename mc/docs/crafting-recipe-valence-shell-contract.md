# Crafting recipe Valence shell contract

## Target scope
This is a selected-matrix Valence shell contract for Java Edition 1.20.1 / protocol 763, shaped `minecraft:chest`, shapeless `minecraft:oak_planks`, invalid `minecraft:stick_insufficient_input_rejection`, and primary-click collection mode. No Valence runtime integration is implemented.

## Inventory and prerequisite evidence
`docs/crafting-recipe-behavior-card.md` `docs/crafting-recipe-selected-matrix-core.md` `docs/crafting-recipe-selected-matrix-data-fixture.md` `docs/crafting-recipe-selected-matrix-receipt-handoff.md` r[vanilla_composable_plugins.crafting_recipe_selected_matrix_receipts] r[valence_bevy_ecs.schedule_hygiene.policy] r[valence_bevy_ecs.gameplay_plugin_contracts.phase_contract] r[valence_bevy_ecs.inventory_sets.contract].

## Core and shell boundary
future ECS systems snapshot crafting-grid state, output-slot state, selected recipe rows, collection requests, client/inventory ownership, and target inventory slots into plain selected-matrix core inputs, call the selected-matrix core, and apply only the returned recipe match, inventory delta, output-blocked decision, no-result decision, or typed malformed-data diagnostic. The pure core must not read files, mutate Bevy world state, write packets, or log.

## Core input mapping
`CraftingGridState` `SelectedRecipeRow` `RecipeMatrix` `OutputSlotState` `CollectionRequest` `CraftingLimits`.

## Core output mapping
`CraftingDecision` `CraftingTransition` `MalformedRecipeError`.

## Opt-in plugin ownership
`CraftingTableShellPlugin` is explicit opt-in with `GameplayInstallMode::ExplicitOptIn` and `GameplayScopeModel::ArenaOwnedLayer`. `CraftingGridResource` `CraftingRecipeTableResource` `CraftingOutputSlotComponent` `CraftingCollectionEvent` `CraftingDiagnosticEvent`.

## Schedule contract
`Update` `GameplayPhase::RuleEvaluation` `GameplayPhase::WorldMutation` `InventoryMutationSet` `InventoryPresentationSet` `FlushPacketsSet` schedule hygiene evidence disabled-plugin comparison.

## Disabled-plugin behavior
When CraftingTableShellPlugin is not installed, no crafting resources are inserted, no crafting events are registered, no crafting systems run, no crafting packets or milestones are emitted, and pure core remains callable by tests.

## Data loading boundary
startup shell or source adapter. Nickel/exported fixture. runtime data-pack parsing is not implemented. Future systems must not parse data packs or read files.

## Mutation, packet, and logging boundaries
snapshot before rule evaluation. single commit step. Packet writes remain outside the core and happen before packet flush. Logging remains outside the core.

## Validation contract
positive validation. negative validation. missing target scope. missing core/shell boundary. missing schedule facts. missing disabled-plugin behavior. DefaultPlugins membership overclaims. all-recipe/collection-mode overclaims.

## Future closeout prerequisites
before any Valence runtime behavior claim: positive runtime tests, negative runtime tests, focused schedule evidence, accepted-spec sync verification, task-evidence validation, evidence-manifest checks.

## Non-claims
No broad Minecraft compatibility. No broad vanilla parity. No all-recipe breadth. No arbitrary collection-mode breadth. No shift-click. No data-pack loading. No recipe-book UI behavior. No automated crafter behavior. No Valence runtime integration. No DefaultPlugins membership change. No public-server safety. No production readiness.