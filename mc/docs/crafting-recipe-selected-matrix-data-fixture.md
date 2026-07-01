# Crafting recipe selected-matrix data fixture

## Scope

This document records the Java Edition 1.20.1 / protocol 763 selected-matrix crafting recipe fixture contract for local unit validation.

The checked fixture is `compat/config/crafting-recipe-selected-matrix-fixture.ncl`, typed by `compat/config/crafting-recipe-selected-matrix-fixture-contracts.ncl`, validated by `tools/check_crafting_recipe_data_fixture.rs`, and handed to the pure core by `tools/check_crafting_recipe_core.rs --self-test --fixture compat/config/crafting-recipe-selected-matrix-fixture.ncl`.

This is selected-row fixture evidence only. It is not all-recipe extraction, Paper/vanilla parity, or Valence runtime behavior.

## Target scope

- Edition: Java Edition.
- Version/protocol: 1.20.1 / protocol 763.
- Fixture breadth: one shaped recipe row, one shapeless recipe row, and one invalid/no-result probe.
- Collection boundary: primary-click collection only.

## Provenance fields

The fixture records:

- `schema` for the fixture contract version.
- `target` edition, game version, and protocol.
- `provenance.data_origin` and `provenance.extraction_status`.
- source entries with name, URL, retrieval date, and evidence role.
- predecessor docs that define the behavior-card and selected-matrix core context.

The Minecraft Wiki source entries guide vocabulary and target-scope review only. They are not treated as Paper/vanilla parity evidence.

## Selected rows and constants

The fixture uses named constants for values consumed by validators and the core handoff:

- `grid_width = 3` and `grid_height = 3`.
- `max_stack_size = 64` and `result_slot_stack_limit = 64`.
- `selected_chest_output_count = 1`.
- `selected_oak_planks_output_count = 4`.
- primary-click target slots `36` and `37` for the selected rows.

Selected shaped row:

- Recipe id: `minecraft:chest`.
- Recipe kind: `shaped`.
- Pattern: `PPP` / `P P` / `PPP`.
- Key: `P = minecraft:oak_planks` count `1`.
- Output: `minecraft:chest` count `1`.

Selected shapeless row:

- Recipe id: `minecraft:oak_planks`.
- Recipe kind: `shapeless`.
- Ingredient: `minecraft:oak_log` count `1`.
- Output: `minecraft:oak_planks` count `4`.

Selected invalid/no-result probe:

- Probe id: `minecraft:stick_insufficient_input_rejection`.
- Probe kind: `rejected_no_result`.
- Input: `minecraft:oak_planks` count `1`.
- Expected diagnostic: `no_result`.

## Validation

`tools/check_crafting_recipe_data_fixture.rs --fixture compat/config/crafting-recipe-selected-matrix-fixture.ncl` verifies:

- Nickel export/type-contract evaluation succeeds.
- Target edition, version, and protocol match Java Edition 1.20.1 / protocol 763.
- Required provenance fields, source entries, and predecessor docs are present.
- Exactly one selected shaped row, one selected shapeless row, and one selected invalid/no-result probe are present.
- Item IDs are valid `minecraft:` identifiers.
- Pattern width, shaped key data, shapeless ingredient data, output counts, grid limits, stack limits, and collection mode are valid selected constants.
- Missing rows, duplicate row IDs, malformed shaped data, malformed shapeless data, invalid item IDs, zero output counts, unsupported recipe kinds, unsupported collection modes, omitted non-claims, and overbroad claim flags fail in self-tests.

The existing core checker consumes the fixture as local handoff evidence with `tools/check_crafting_recipe_core.rs --self-test --fixture compat/config/crafting-recipe-selected-matrix-fixture.ncl`. The handoff checks shaped matching, shapeless matching, primary-click inventory deltas, and invalid/no-result preservation using the fixture rows.

## Non-claims

This fixture does not claim:

- Broad Minecraft compatibility.
- Broad vanilla parity.
- Paper/vanilla parity.
- All-recipe breadth.
- Arbitrary collection modes.
- Shift-click, drag, or split handling.
- Data-pack loading.
- Recipe-book UI behavior.
- Recipe discovery, advancement, or `doLimitedCrafting` behavior.
- Automated crafter behavior.
- Valence runtime integration.
- DefaultPlugins membership changes.
- Public-server safety.
- Production readiness.

## Follow-on evidence

Before promoting stronger target-version or runtime behavior, follow-on work still needs selected receipt handoff against Paper/reference and Valence evidence, a Valence shell contract, opt-in runtime shell tests, and separate scopes for all-recipe breadth, data-pack loading, recipe-book behavior, automated crafter behavior, and additional collection modes.
