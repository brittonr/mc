# Crafting recipe selected-matrix receipt handoff

## Scope

This document defines the deterministic bridge from the selected Java Edition 1.20.1 / protocol 763 crafting fixture to archived Paper/reference and Valence receipt evidence.

It does not add a new live run. The handoff reuses the archived `survival-crafting-recipe-breadth` Paper/reference and Valence normalized key/value evidence only after `tools/check_crafting_recipe_receipt_handoff.rs` verifies that the fixture rows and receipt metrics describe the same selected matrix.

## Inputs

- Fixture: `compat/config/crafting-recipe-selected-matrix-fixture.ncl`.
- Paper/reference evidence: `docs/evidence/survival-crafting-recipe-breadth-paper-2026-06-20.kv`.
- Valence evidence: `docs/evidence/survival-crafting-recipe-breadth-valence-2026-06-20.kv`.
- Checker: `tools/check_crafting_recipe_receipt_handoff.rs`.

The checker first runs `nickel export` on the fixture, then reads the fixture text and the two normalized receipt inputs. The comparison core receives only in-memory fixture and receipt values.

## Normalized field mapping

| Fixture value | Paper/Valence receipt metric | Required value or rule |
| --- | --- | --- |
| `target_edition`, `target_game_version`, `target_protocol` | `metric.matrix.version` plus receipt row context | Fixture target must be Java Edition 1.20.1 / protocol 763; receipt matrix version must be `2026-06-20`. |
| `selected_chest_recipe_id` | `metric.matrix.shaped.recipe_id` | `minecraft:chest`. |
| Chest pattern/key | `metric.matrix.shaped.input_slots` | Slots `1,2,3,4,6,7,8,9` must normalize to `minecraft:oak_planks:1`. |
| `selected_chest_output_item`, `selected_chest_output_count_value` | `metric.matrix.shaped.result_item`, `metric.matrix.shaped.result_count`, `metric.matrix.shaped.final_inventory_item`, `metric.matrix.shaped.final_inventory_count` | Output and collected inventory stack must normalize to `minecraft:chest` count `1`. |
| `selected_chest_target_slot_value` | `metric.matrix.shaped.final_inventory_slot` | Slot `36`. |
| `selected_collection_mode` | `metric.matrix.shaped.collection_mode`, `metric.matrix.collection_modes` | `primary_click` only. |
| `selected_oak_planks_recipe_id` | `metric.matrix.shapeless.recipe_id` | `minecraft:oak_planks`. |
| `selected_shapeless_input_item`, `selected_shapeless_input_count` | `metric.matrix.shapeless.input_slots` | Slot `1` must normalize to `minecraft:oak_log:1`. |
| `selected_shapeless_output_item`, `selected_oak_planks_output_count_value` | `metric.matrix.shapeless.result_item`, `metric.matrix.shapeless.result_count`, `metric.matrix.shapeless.final_inventory_item`, `metric.matrix.shapeless.final_inventory_count` | Output and collected inventory stack must normalize to `minecraft:oak_planks` count `4`. |
| `selected_oak_planks_target_slot_value` | `metric.matrix.shapeless.final_inventory_slot` | Slot `37`. |
| `selected_invalid_probe_id` | `metric.matrix.invalid.recipe_id` | `minecraft:stick_insufficient_input_rejection`. |
| `selected_invalid_probe_input_item`, `selected_invalid_probe_input_count` | `metric.matrix.invalid.input_slots` | Slot `1` must normalize to `minecraft:oak_planks:1`. |
| `selected_invalid_probe_diagnostic` | `metric.matrix.invalid.result_item`, `metric.matrix.invalid.result_count`, `metric.matrix.invalid.rejection_outcome` | Empty result (`None`/`minecraft:none`), count `0`, outcome `no_result`. |
| Receipt fields `receipt`, `typed_events`, `client_log`, `server_log`, `run_log` | same key/value fields | Paths must be present and stay under `docs/evidence/`. |
| Receipt fields `backend`, `revision_status`, `child_revision` | same key/value fields | Paper evidence must identify backend `paper`; Valence evidence must identify backend `valence`; both must have clean revision metadata. |
| Fixture non-claims and receipt nonclaim metrics | `metric.nonclaim.*` plus fixture `non_claims` | Required non-claims must remain present; broad `claim.*=true` overclaims fail. |

Receipt item aliases accepted by the checker are intentionally explicit: `OakPlanks`, `OakLog`, `Chest`, and `None` normalize to the corresponding `minecraft:` item or empty-result sentinel. Unknown aliases fail closed.

## Checker coverage

The checker self-test includes one positive selected-matrix handoff and negative cases for:

- missing Paper evidence;
- missing Valence evidence;
- stale or wrong row ids;
- malformed receipt rows;
- mismatched shaped inputs;
- mismatched shapeless inputs;
- mismatched output items;
- mismatched output counts;
- mismatched target inventory slots;
- unsupported collection modes;
- missing non-claim metrics;
- broad crafting overclaims;
- missing fixture non-claims.

## Evidence boundary

A passing checker proves only that the validated selected fixture and the archived Paper/reference plus Valence receipt metrics agree for the selected shaped chest row, selected shapeless oak-planks row, selected invalid/no-result probe, primary-click collection mode, target inventory slots, backend identity, clean revision metadata, receipt paths, and retained non-claim fields.

It does not prove all recipes, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book behavior, recipe discovery or advancement behavior, automated crafter behavior, Valence runtime integration, default plugin membership, broad vanilla parity, broad Minecraft compatibility, public-server safety, or production readiness.
