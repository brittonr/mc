# Behavior card: crafting recipe selected matrix

## Source pages

Retrieval date: 2026-07-01.

- Crafting: https://minecraft.wiki/w/Crafting
- Recipe (Java Edition): https://minecraft.wiki/w/Recipe_(Java_Edition)
- Java Edition 1.20.1: https://minecraft.wiki/w/Java_Edition_1.20.1
- Protocol version: https://minecraft.wiki/w/Protocol_version

These Minecraft Wiki pages are untrusted external guidance only. They identify vocabulary, behavior seams, Java Edition recipe data shapes, and version-drift risk; they do not prove target-version vanilla behavior.

Version-drift risk is high for crafting pages because current wiki text includes newer release trains, data-pack behavior, recipe-book behavior, and automated crafter behavior beyond this first selected slice. Target-version extracted data and paired receipt evidence remain required before implementation claims.

## Target scope

- Edition: Java Edition.
- Version/protocol: 1.20.1 / protocol 763.
- Data source required before behavior claims: target-version recipe JSON extracted from the Java Edition 1.20.1 jar for the selected shaped and shapeless rows, plus explicit malformed-data fixtures for rejected rows.
- Compatibility predecessor: `survival-crafting-recipe-breadth` paired Paper/reference and Valence receipts, plus typed-event migration evidence for the same row.
- Local repo scope: future Valence plugin work; no default Valence plugin membership change in this behavior-card-only package.

## Selected recipe matrix

The first implementation slice is finite and selected:

| Row | Recipe id | Shape | Inputs | Expected result | Collection mode | Predecessor evidence |
| --- | --- | --- | --- | --- | --- | --- |
| shaped chest | `minecraft:chest` | shaped 3×3 ring | eight `minecraft:oak_planks` inputs in slots `1,2,3,4,6,7,8,9` | `minecraft:chest` count `1` | primary-click collection mode into inventory slot `36` | `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md` |
| shapeless oak-planks | `minecraft:oak_planks` | shapeless | one `minecraft:oak_log` input in slot `1` | `minecraft:oak_planks` count `4` | primary-click collection mode into inventory slot `37` | `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md` |
| invalid stick rejection | `minecraft:stick_insufficient_input_rejection` | rejected/no-result | one `minecraft:oak_planks` input in slot `1` | empty result slot with `no_result` diagnostic | no collection | `docs/evidence/survival-crafting-recipe-breadth-receipts-2026-06-20.md` |

The existing row receipts are predecessor receipt evidence only. They do not implement a reusable recipe core or Valence shell.

## Bounded claim

Future evidence may promote only this bounded claim: a selected crafting recipe matrix for Java Edition 1.20.1 / protocol 763 matches one shaped chest recipe, matches one shapeless oak-planks recipe, rejects one insufficient stick-input case, and handles one primary-click collection mode with deterministic result or rejection diagnostics.

This package does not implement the rule core and does not claim vanilla parity.

## Non-claims

- No broad Minecraft compatibility.
- No broad vanilla parity.
- No full survival correctness.
- No all-recipe breadth until all target-version recipe rows are extracted and tested.
- No arbitrary collection modes.
- No shift-click, drag, or split breadth.
- No data-pack loading.
- No recipe-book UI behavior.
- No recipe discovery, advancement, or `doLimitedCrafting` behavior.
- No automated crafter behavior.
- No hopper automation or redstone-driven crafting behavior.
- No public-server safety.
- No production readiness.
- No DefaultPlugins membership change.

## Pure recipe core

Inputs:

- `CraftingGridState` containing the in-memory crafting grid slots, item ids, stack counts, and optional item metadata needed by the selected rows.
- `RecipeMatrix` containing target-version selected rows only.
- `SelectedRecipeRow` for `minecraft:chest`, `minecraft:oak_planks`, or `minecraft:stick_insufficient_input_rejection` with recipe kind, pattern or shapeless ingredients, output id, output count, and target version metadata.
- `OutputSlotState` containing current result-slot contents and stack limit facts.
- `CollectionRequest`, initially limited to primary-click collection mode.
- Named constants or data fields for grid width, grid height, maximum stack size, and result-slot stack limits from target-version data.

Outputs:

- `CraftingDecision::Matched` with recipe id, output item id/count, remaining grid state, and a proposed inventory delta for the collection request.
- `CraftingDecision::NoResult` for invalid or insufficient input without mutating the grid or inventory.
- `CraftingDecision::OutputBlocked` for incompatible or full output-slot state.
- `MalformedRecipeError` for malformed selected recipe rows, unsupported recipe kinds, missing target-version data, duplicate row ids, invalid item ids, zero output counts, or unsupported collection modes.

Error cases:

- Missing selected recipe row.
- Malformed shaped pattern or key mapping.
- Malformed shapeless ingredient list.
- Unexpected item id or count in the grid.
- Output slot blocked by a wrong item or full stack.
- Missing target-version recipe JSON.
- Unsupported `CollectionRequest` such as shift-click, drag, or split handling.
- Data-pack replacement or recipe-book-only state reaching the selected-row core.

The pure deterministic recipe core must not read files, fetch wiki pages, inspect environment variables, query Bevy resources, mutate Bevy world state, emit packets, write logs, parse data packs, access the network, or depend on wall-clock time.

## Thin Bevy/ECS shell

Resources/components/events owned:

- Player inventory and open-screen snapshots, including crafting grid slots and result slot.
- Packet-derived inventory click events and collection requests.
- Selected recipe data resource loaded by a separate startup/data-loading boundary.
- Optional typed crafting diagnostic event and inventory update event after state commit.

Systems:

- A future `crafting_recipe_click_system` adapts an inventory click event into `CraftingGridState`, `OutputSlotState`, `RecipeMatrix`, and `CollectionRequest`, calls the pure recipe core, commits only the returned grid/inventory deltas, and emits only documented diagnostics/events.

Schedule phase and ordering:

- Runs in an inventory click or packet event-loop phase after packet decoding and before client-visible inventory update emission.
- Must run after the selected recipe data resource is loaded.
- Must record any new schedule phase, ordering dependency, or default-membership proposal before implementation archive.
- Must include disabled-plugin behavior: when the opt-in crafting recipe plugin is absent, no selected-row crafting mutation, packet emission, or diagnostic event is introduced by this shell.

I/O and mutation boundary:

- ECS shell owns Bevy queries, resource reads, world mutation, inventory packet/event emission, and disabled-plugin checks.
- Core owns shaped/shapeless match semantics, result selection, rejected/no-result decisions, output-slot blocking decisions, and malformed-data diagnostics.
- The shell does not parse data packs, decide recipe semantics, fetch network pages, read filesystem data inside the click system, or broaden collection semantics beyond the selected request.

## Tests

Positive tests:

- `shaped chest`: eight oak-planks inputs in the selected ring match `minecraft:chest` and produce one chest.
- `shapeless oak-planks`: one oak log in the selected grid matches `minecraft:oak_planks` and produces four oak planks.
- `primary-click collection`: a matched selected row with compatible inventory capacity produces the expected output and grid/inventory delta without hidden side effects.

Negative tests:

- `insufficient stick input`: one oak-planks input for the stick rejection row returns `NoResult` with an empty result slot.
- `output slot blocked`: a selected recipe with an incompatible or full result slot returns `OutputBlocked` and preserves the grid.
- `malformed recipe row`: invalid shaped pattern/key data, invalid shapeless ingredient data, missing target-version data, invalid item ids, or zero output counts fail before matching.
- `unsupported collection mode`: shift-click, drag, split, automated crafter, or recipe-book UI requests fail closed until separately scoped.
- Missing recipe data, duplicate selected recipe ids, and unsupported recipe kinds fail with deterministic diagnostics.

## Evidence

Required before implementation claims:

- Focused behavior-card validation with positive and negative self-tests.
- Extracted-data check for Java Edition 1.20.1 target-version recipe JSON covering `minecraft:chest` and `minecraft:oak_planks` and malformed selected rows.
- Positive and negative pure-core tests for the selected matrix.
- Paper/reference and Valence receipts or accepted vanilla-reference evidence for the selected matrix before promoting behavior beyond local unit semantics.
- Existing `survival-crafting-recipe-breadth` row evidence and typed-event migration evidence may be used as predecessor vocabulary only.
- mc-compat row or evidence note that records target version, tested inputs, rejected invalid inputs, collection mode, and explicit non-claims.

Evidence in this behavior-card-only package may claim only that the card is complete enough to seed a follow-on implementation Cairn. It does not claim the pure recipe core, the Valence shell, all recipes, data-pack loading, recipe-book UI behavior, automated crafter behavior, default plugin membership, public-server safety, production readiness, broad Minecraft compatibility, or broad vanilla parity.

## Stop conditions before broader work

- Stop before all-recipe breadth until target-version recipe extraction exists and all promoted rows have positive and negative coverage.
- Stop before data-pack loading until a dedicated data-pack recipe-loading card, loader contract, conflict/replacement tests, and malformed-pack negatives exist.
- Stop before recipe-book UI behavior until recipe discovery, unlock, `doLimitedCrafting`, UI packet, and client-visible synchronization evidence is scoped.
- Stop before automated crafter behavior until a separate automated-crafter behavior card covers redstone activation, disabled slots, hopper insertion, and scheduling.
- Stop before arbitrary collection modes, shift-click, drag, or split breadth until each collection mode has dedicated tests and receipt evidence.
- Stop before Valence runtime behavior until the pure core, target-version data fixture, shell contract, disabled-plugin behavior, and focused shell tests exist.
- Stop before default plugin membership changes until schedule impact and compatibility evidence are recorded.
