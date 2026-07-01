# Furnace smelting selected-row data fixture

## Scope

This document records the Java Edition 1.20.1 / protocol 763 selected-row furnace fixture contract for local unit validation.

The checked fixture is `compat/config/furnace-smelting-selected-row-fixture.ncl`, typed by `compat/config/furnace-smelting-selected-row-fixture-contracts.ncl` and validated by `tools/check_furnace_smelting_data_fixture.rs`.

## Target scope

- Edition: Java Edition.
- Version/protocol: 1.20.1 / protocol 763.
- Furnace kind: standard furnace only.
- Fixture breadth: one selected recipe row and one selected fuel row.

## Provenance fields

The fixture records:

- `schema` for the fixture contract version.
- `target` edition, game version, and protocol.
- `provenance.data_origin` and `provenance.extraction_status`.
- source entries with name, URL, retrieval date, and evidence role.
- predecessor docs that define the behavior-card and selected-row core context.

The Minecraft Wiki source entries guide vocabulary and target-scope review only. They are not treated as Paper/vanilla parity evidence.

## Selected rows and constants

The fixture uses named constants for values consumed by validators and the core handoff:

- `standard_furnace_cook_ticks = 200`.
- `coal_burn_ticks = 1600`.
- `max_stack_size = 64`.
- `selected_recipe_output_count = 1`.

Selected recipe row:

- Furnace kind: `standard`.
- Input item: `minecraft:raw_iron`.
- Output item: `minecraft:iron_ingot`.
- Output count: `selected_recipe_output_count`.
- Cook ticks: `standard_furnace_cook_ticks`.

Selected fuel row:

- Fuel item: `minecraft:coal`.
- Burn ticks: `coal_burn_ticks`.

## Validation

`tools/check_furnace_smelting_data_fixture.rs --fixture compat/config/furnace-smelting-selected-row-fixture.ncl` verifies:

- Nickel export/type-contract evaluation succeeds.
- Target edition, version, and protocol match Java Edition 1.20.1 / protocol 763.
- Exactly one selected standard-furnace recipe row and one selected fuel row are present.
- Item IDs are valid `minecraft:` identifiers.
- Output counts, cook ticks, burn ticks, and stack limits are positive named constants with expected selected-row values.
- Unsupported furnace kinds, malformed rows, missing rows, and omitted non-claims fail in self-tests.

The existing core checker can consume the fixture as local handoff evidence with `tools/check_furnace_smelting_core.rs --self-test --fixture compat/config/furnace-smelting-selected-row-fixture.ncl`.

## Non-claims

This fixture does not claim:

- Broad Minecraft compatibility.
- Broad vanilla parity.
- Paper/vanilla parity.
- All-recipe breadth.
- Smoker behavior.
- Blast-furnace behavior.
- Hopper automation.
- XP behavior.
- Recipe-book synchronization.
- Chunk-unload semantics.
- Valence runtime integration.
- DefaultPlugins membership changes.
- Public-server safety.
- Production readiness.

## Receipt handoff

`docs/furnace-smelting-selected-row-receipt-handoff.md` records the deterministic bridge from this fixture to archived Paper/reference and Valence receipt metrics. The handoff checker validates the selected RawIron + Coal row, timing constants, backend identities, and retained non-claims before the archived receipts are reused as selected-row evidence. `docs/furnace-smelting-valence-shell-contract.md` is the follow-on shell boundary for mapping this fixture-backed core into future opt-in Valence runtime work.

## Follow-on evidence

Before promoting stronger target-version behavior, follow-on work still needs all-recipe extraction if breadth is claimed, the Valence Bevy/ECS shell contract in `docs/furnace-smelting-valence-shell-contract.md` plus focused schedule evidence if runtime behavior is claimed, and separate cards for hoppers, XP, recipe book, smoker, blast furnace, and chunk-unload semantics. The receipt handoff does not add a runtime shell or broaden the selected-row claim.
