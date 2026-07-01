# Furnace smelting selected-row receipt handoff evidence — 2026-07-01

## Scope

This checkpoint closes the selected-row receipt handoff for the RawIron + Coal standard-furnace row. It reuses archived `survival-furnace-smelting-breadth` Paper/reference and Valence normalized receipt inputs only after the handoff checker validates that they match the selected fixture.

## Inputs checked

- Fixture: `compat/config/furnace-smelting-selected-row-fixture.ncl`.
- Paper/reference normalized receipt: `docs/evidence/survival-furnace-smelting-breadth-paper-2026-06-21.kv`.
- Valence normalized receipt: `docs/evidence/survival-furnace-smelting-breadth-valence-2026-06-21.kv`.
- Handoff checker: `tools/check_furnace_smelting_receipt_handoff.rs`.

## Result

The handoff checker passed and recorded:

- Target: Java Edition 1.20.1 / protocol 763.
- Receipt row: `survival-furnace-smelting-breadth-parity`.
- Input: `minecraft:raw_iron`.
- Fuel: `minecraft:coal`.
- Output: `minecraft:iron_ingot` x1.
- Cook ticks: 200.
- Burn ticks: 1600.
- Paper/reference evidence revision: `66c0f31c620b52258d9a424a03375e4b6ef708f8`.
- Valence evidence revision: `53ec70c527796b158463d087fbbb9d0826bc52c5`.

## Validation logs

- Baseline fixture/core validation: `docs/evidence/furnace-smelting-selected-row-receipts-baseline-2026-07-01.run.log` (`exit_status=0`).
- Checker positive and negative self-tests: `docs/evidence/furnace-smelting-selected-row-receipts-checker-self-test-2026-07-01.run.log` (`exit_status=0`).
- Selected-row handoff validation: `docs/evidence/furnace-smelting-selected-row-receipts-handoff-2026-07-01.run.log` (`exit_status=0`).
- Rust formatting: `docs/evidence/furnace-smelting-selected-row-receipts-rustfmt-2026-07-01.run.log` (`exit_status=0`).
- Post-format focused checker self-test plus handoff validation: `docs/evidence/furnace-smelting-selected-row-receipts-focused-validation-2026-07-01.run.log` (`exit_status=0`).
- Cairn gates and validation: `docs/evidence/furnace-smelting-selected-row-receipts-cairn-gates-2026-07-01.run.log` (`exit_status=0`).
- Task-evidence gate after implementation tasks: `docs/evidence/furnace-smelting-selected-row-receipts-task-evidence-initial-2026-07-01.run.log` (`exit_status=0`).
- Accepted-spec sync dry-run and execution: `docs/evidence/furnace-smelting-selected-row-receipts-sync-dry-run-2026-07-01.run.log` and `docs/evidence/furnace-smelting-selected-row-receipts-sync-execute-2026-07-01.run.log` (`exit_status=0`).
- Accepted-spec ID verification and Cairn validation: `docs/evidence/furnace-smelting-selected-row-receipts-accepted-spec-verify-2026-07-01.run.log` (`exit_status=0`).
- Evidence-manifest refresh/check: `docs/evidence/furnace-smelting-selected-row-receipts-evidence-manifest-refresh-2026-07-01.run.log` (`exit_status=0`).
- Flake evidence checks: `docs/evidence/furnace-smelting-selected-row-receipts-flake-evidence-checks-2026-07-01.run.log` and `docs/evidence/furnace-smelting-selected-row-receipts-flake-evidence-checks-final-2026-07-01.run.log` (`exit_status=0`).
- Archive dry-run and execution: `docs/evidence/furnace-smelting-selected-row-receipts-archive-dry-run-2026-07-01.run.log` and `docs/evidence/furnace-smelting-selected-row-receipts-archive-execute-2026-07-01.run.log` (`exit_status=0`).
- Post-archive validation: `docs/evidence/furnace-smelting-selected-row-receipts-post-archive-validate-2026-07-01.run.log` (`exit_status=0`, active changes = 0).
- Post-archive manifest/task-evidence checks: `docs/evidence/furnace-smelting-selected-row-receipts-post-archive-evidence-manifest-check-2026-07-01.run.log`, `docs/evidence/furnace-smelting-selected-row-receipts-post-archive-task-evidence-2026-07-01.run.log`, and `docs/evidence/furnace-smelting-selected-row-receipts-post-archive-flake-evidence-checks-2026-07-01.run.log` (`exit_status=0`).

BLAKE3 coverage is recorded in `docs/evidence/furnace-smelting-selected-row-receipts-2026-07-01.b3`.

## Non-claims

This handoff does not claim all recipes, all fuels, smoker behavior, blast-furnace behavior, hopper automation, XP behavior, recipe-book synchronization, chunk-unload semantics, Valence runtime integration, DefaultPlugins membership changes, broad Minecraft compatibility, broad vanilla parity, public-server safety, or production readiness. No new live Paper/Valence run is implied by this checkpoint.
