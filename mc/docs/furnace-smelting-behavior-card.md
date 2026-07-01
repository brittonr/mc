# Behavior card: furnace smelting selected row

## Source pages

Retrieval date: 2026-07-01.

- Smelting: https://minecraft.wiki/w/Smelting
- Block entity: https://minecraft.wiki/w/Block_entity
- Java Edition 1.20.1: https://minecraft.wiki/w/Java_Edition_1.20.1

These Minecraft Wiki pages are untrusted external guidance only. They identify vocabulary, behavior seams, and version-drift risks; they do not prove target-version vanilla behavior.

## Target scope

- Edition: Java Edition.
- Version/protocol: 1.20.1 / protocol 763.
- Data source required before behavior claims: target-version recipe and fuel extraction plus selected Paper/vanilla furnace receipts.
- Local repo scope: future Valence plugin work; no default Valence plugin membership change in this behavior-card-only package.

## Bounded claim

Future evidence may promote only this bounded claim: a selected standard furnace row advances one smeltable input into one compatible output when a valid fuel source and output slot capacity are present under Java Edition 1.20.1 rules.

This package does not implement the rule core and does not claim vanilla parity.

## Non-claims

- No broad Minecraft compatibility.
- No broad vanilla parity.
- No full survival correctness.
- No all-recipe breadth until all target-version recipe rows are extracted and tested.
- No all block entities, all container behavior, or hopper automation.
- No XP rounding, smoker category breadth, blast-furnace category breadth, recipe-book synchronization, or chunk-unload semantics.
- No public-server safety.
- No production readiness.
- No DefaultPlugins membership change.

## Pure rule core

Inputs:

- `FurnaceKind`, initially limited to `standard` for the first implementation slice.
- `FurnaceState` containing input stack, fuel stack, output stack, `cook_progress_ticks`, `remaining_burn_ticks`, and accumulated recipe counters.
- `RecipeTable` filtered to Java Edition 1.20.1.
- `FuelTable` filtered to Java Edition 1.20.1.
- Named constants for `standard_furnace_cook_ticks` and maximum stack size from target-version data.

Outputs:

- New `FurnaceState`.
- `FurnaceTransition` diagnostic such as `StartedFuel`, `AdvancedCooking`, `ProducedOutput`, `PausedNoFuel`, `PausedNoRecipe`, or `PausedOutputBlocked`.

Error cases:

- Missing recipe table row.
- Fuel item absent or invalid.
- Output stack full.
- Output stack item kind mismatch.
- Malformed extracted recipe row.
- Unsupported `FurnaceKind` for the first bounded slice.

The pure deterministic rule core must not read files, fetch wiki pages, inspect environment variables, write logs, emit packets, mutate Bevy world state, or depend on wall-clock time.

## Thin Bevy/ECS shell

Resources/components/events owned:

- Furnace block-entity component snapshot.
- Inventory slot access for input, fuel, and output slots.
- Recipe and fuel data resources loaded by a separate data-loading boundary.
- Optional inventory or layer update event emitted after state commit.

Systems:

- A future `furnace_smelt_tick_system` reads ECS state, calls the pure core with in-memory values, writes back the returned state, and emits only returned diagnostics/events.

Schedule phase and ordering:

- Runs in a named block-entity tick phase.
- Must run after recipe/fuel data resources are available.
- Must run before client-visible inventory or layer update emission.
- Must record any new schedule phase or ordering dependency before implementation archive.

I/O and mutation boundary:

- ECS shell owns Bevy queries, resource reads, world mutation, and packet/event emission.
- Core owns recipe/fuel semantic decisions.
- The shell does not parse data packs, decide recipe semantics, fetch network pages, or read filesystem data inside the tick system.

## Tests

Positive tests:

- Standard furnace consumes one valid fuel and advances a valid smeltable recipe.
- Output merges into a compatible non-full stack.
- Remaining burn time advances cooking without consuming another fuel item.
- Completed cook produces exactly the selected output row and resets cook progress.

Negative tests:

- Invalid input produces `PausedNoRecipe` without consuming fuel.
- Empty fuel with no remaining burn produces `PausedNoFuel`.
- Wrong output item produces `PausedOutputBlocked` and preserves input.
- Full output stack produces `PausedOutputBlocked`.
- Malformed extracted recipe row fails validation before the core is called.
- Unsupported furnace kind fails before broader smoker or blast-furnace claims are promoted.

## Evidence

Required before implementation claims:

- Focused behavior-card validation with positive and negative self-tests.
- Extracted-data check for Java Edition 1.20.1 recipes and fuels.
- Positive and negative pure-core tests for the selected row.
- Paper/vanilla receipt for at least one selected standard-furnace scenario.
- mc-compat row or evidence note that records target version, tested inputs, rejected invalid inputs, and explicit non-claims.

Evidence in this behavior-card-only package may claim only that the card is complete enough to seed a follow-on implementation Cairn. Later selected-row receipt handoff evidence is tracked separately in `docs/furnace-smelting-selected-row-receipt-handoff.md`; it bridges one fixture row to archived Paper/reference and Valence receipts without adding a Valence runtime shell or breadth claim.

## Stop conditions before broader work

- Stop before all-recipe breadth until target-version recipe extraction exists.
- Stop before smoker and blast-furnace category claims until category-specific extracted data exists.
- Stop before hoppers, XP, recipe book, data packs, or chunk-unload behavior until each has a dedicated behavior card and parity receipt.
- Stop before default plugin membership changes until schedule impact and compatibility evidence are recorded.
