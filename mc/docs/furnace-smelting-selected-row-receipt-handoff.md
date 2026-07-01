# Furnace smelting selected-row receipt handoff

## Scope

This document records the deterministic bridge between the selected Java Edition 1.20.1 / protocol 763 fixture row and the archived Paper/reference plus Valence survival furnace receipt evidence.

The handoff is implemented by `tools/check_furnace_smelting_receipt_handoff.rs`. The checker has a pure comparison core over in-memory fixture and receipt rows, plus a thin shell that runs Nickel export for the fixture and reads normalized `.kv` receipt inputs.

## Inputs

Fixture input:

- `compat/config/furnace-smelting-selected-row-fixture.ncl`

Archived receipt inputs reused by this handoff:

- Paper/reference: `docs/evidence/survival-furnace-smelting-breadth-paper-2026-06-21.kv`
- Valence: `docs/evidence/survival-furnace-smelting-breadth-valence-2026-06-21.kv`

No new live Paper, Valence, Docker, or Stevenarella run is implied by this handoff. It reuses the archived `survival-furnace-smelting-breadth` artifacts only because the handoff checker validates the selected row against both normalized receipt inputs.

## Normalized field mapping

| Contract field | Fixture source | Receipt source | Expected selected-row value |
| --- | --- | --- | --- |
| Target edition | `target.edition` | Handoff scope | Java Edition |
| Target game version | `target.game_version` | Handoff scope | 1.20.1 |
| Target protocol | `target.protocol` | Handoff scope | 763 |
| Furnace kind | selected recipe `furnace_kind` | Handoff scope | standard |
| Receipt row | handoff contract | `row` | `survival-furnace-smelting-breadth-parity` |
| Backend identity | handoff contract | `backend` | Paper/reference `paper`, Valence `valence` |
| Revision status | handoff contract | `revision_status` and `child_revision` | clean, non-placeholder revision |
| Matrix version | handoff contract | `metric.matrix.version` | `2026-06-20` |
| Input item | `selected_recipe_input_item` | `metric.smelt.input_item` | `minecraft:raw_iron` / accepted alias `RawIron` |
| Fuel item | `selected_fuel_item` | `metric.smelt.fuel_item` | `minecraft:coal` / accepted alias `Coal` |
| Output item | `selected_recipe_output_item` | `metric.smelt.output_item` | `minecraft:iron_ingot` / accepted alias `IronIngot` |
| Output count | `selected_recipe_output_count_value` | `metric.smelt.output_count` | 1 |
| Cook ticks | `selected_standard_furnace_cook_ticks` | `metric.smelt.cook_ticks` | 200 |
| Burn ticks | `selected_coal_burn_ticks` | `metric.smelt.burn_ticks` | 1600 |
| Invalid-fuel guard | selected input item | `metric.invalid_fuel.item` and `metric.invalid_fuel.outcome` | `RawIron` rejected as `no_burn` |
| Broad-furnace non-claim | fixture non-claims | `metric.nonclaim.all_furnaces` and absent true `claim.*` rows | `true` non-claim and no broad claim |

The checker accepts only explicit item aliases used by the archived receipts. Ambiguous or unknown item labels fail closed.

## What the handoff proves

The successful handoff proves that the validated selected fixture row and the archived Paper/reference plus Valence normalized receipt rows describe the same bounded standard-furnace case:

- RawIron input.
- Coal fuel.
- IronIngot output.
- Output count 1.
- Cook time 200 ticks.
- Coal burn time 1600 ticks.
- Clean backend evidence rows for both Paper/reference and Valence.
- Required selected-row non-claim boundaries retained.

## Non-claims retained

The follow-on shell boundary is `docs/furnace-smelting-valence-shell-contract.md`. It maps this selected-row evidence to future opt-in Valence runtime planning and remains a prerequisite before any runtime behavior claim.

This handoff does not claim:

- Broad Minecraft compatibility.
- Broad vanilla parity.
- Paper/vanilla parity beyond the reused selected-row receipt bridge.
- All-recipe breadth.
- All-fuel breadth.
- Smoker behavior.
- Blast-furnace behavior.
- Hopper automation.
- XP behavior.
- Recipe-book synchronization.
- Chunk-unload semantics.
- Valence runtime integration.
- Valence shell implementation.
- DefaultPlugins membership changes.
- Public-server safety.
- Production readiness.

## Validation evidence

- Baseline fixture/core validation: `docs/evidence/furnace-smelting-selected-row-receipts-baseline-2026-07-01.run.log`.
- Checker positive and negative self-tests: `docs/evidence/furnace-smelting-selected-row-receipts-checker-self-test-2026-07-01.run.log`.
- Selected-row handoff validation: `docs/evidence/furnace-smelting-selected-row-receipts-handoff-2026-07-01.run.log`.
- Post-format focused checker self-test plus handoff validation: `docs/evidence/furnace-smelting-selected-row-receipts-focused-validation-2026-07-01.run.log`.
- Cairn gates, accepted-spec sync, evidence-manifest refresh/check, and flake evidence checks are summarized in `docs/evidence/furnace-smelting-selected-row-receipts-2026-07-01.md`.

BLAKE3 coverage is recorded in `docs/evidence/furnace-smelting-selected-row-receipts-2026-07-01.b3`.

## Shell-contract prerequisite

`docs/furnace-smelting-valence-shell-contract.md` records the opt-in Bevy/ECS shell contract, disabled-plugin behavior, schedule boundary, mutation boundary, and validation prerequisites that must be satisfied before this selected-row handoff is used for Valence runtime behavior claims.
