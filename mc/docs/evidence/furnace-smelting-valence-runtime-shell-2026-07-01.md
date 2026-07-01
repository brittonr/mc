# Furnace smelting Valence runtime shell evidence summary

## Scope

This evidence summary covers the selected-row opt-in Valence runtime shell for standard-furnace RawIron + Coal -> IronIngot behavior.

## Promoted artifacts

- Runtime-shell docs: `docs/furnace-smelting-valence-runtime-shell.md`.
- Runtime-shell inventory: `docs/evidence/furnace-smelting-valence-runtime-shell-inventory-2026-07-01.md`.
- Baseline log: `docs/evidence/furnace-smelting-valence-runtime-shell-baseline-2026-07-01.run.log`.
- Focused shell test log: `docs/evidence/furnace-smelting-valence-runtime-shell-focused-validation-2026-07-01.run.log`.
- Schedule hygiene log: `docs/evidence/furnace-smelting-valence-runtime-shell-schedule-hygiene-2026-07-01.run.log`.
- Affected Valence example test log: `docs/evidence/furnace-smelting-valence-runtime-shell-valence-example-2026-07-01.run.log`.
- Format log: `docs/evidence/furnace-smelting-valence-runtime-shell-rustfmt-2026-07-01.run.log`.
- Accepted-spec sync logs: `docs/evidence/furnace-smelting-valence-runtime-shell-sync-dry-run-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-sync-execute-2026-07-01.run.log`, and `docs/evidence/furnace-smelting-valence-runtime-shell-accepted-spec-verify-2026-07-01.run.log`.
- Cairn gate/validation log: `docs/evidence/furnace-smelting-valence-runtime-shell-cairn-gates-2026-07-01.run.log`.
- Task-evidence and evidence-manifest final logs: `docs/evidence/furnace-smelting-valence-runtime-shell-task-evidence-final-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-archive-task-evidence-2026-07-01.run.log`, and `docs/evidence/furnace-smelting-valence-runtime-shell-evidence-manifest-final-check-2026-07-01.run.log`.
- Archive and post-archive receipts: `docs/evidence/furnace-smelting-valence-runtime-shell-archive-dry-run-2026-07-01.run.log`, `docs/evidence/furnace-smelting-valence-runtime-shell-archive-execute-2026-07-01.run.log`, and `docs/evidence/furnace-smelting-valence-runtime-shell-post-archive-cairn-validate-2026-07-01.run.log`.

## Checks represented

- Fixture validation, selected-row pure core self-test, receipt-handoff validation, and shell-contract validation passed with `overall_exit_status=0` in the baseline log.
- `cargo test --example survival_compat furnace -- --nocapture` passed with 18 selected furnace tests, including positive fuel start, active burn progress, output production, compatible merge, and negative invalid/no fuel/blocked/unsupported/malformed/stale/disabled-plugin cases.
- `cargo test --example survival_compat -- --nocapture` passed with 94 example tests as the affected Valence example check.
- `tools/check_valence_schedule_hygiene.rs --root .` passed with `overall_exit_status=0`.
- `cargo fmt --all -- --check` passed with `overall_exit_status=0` after formatting.
- Cairn sync dry-run, sync execute, and accepted-spec verification passed with `overall_exit_status=0`.
- Cairn proposal/design/tasks gates and validation passed with `overall_exit_status=0`.
- Task-evidence validation before archive, archived task-evidence validation, evidence-manifest final check, archive dry-run, archive execute, and post-archive Cairn validation passed with `overall_exit_status=0`; post-archive validation reported `changes: 0`.

## Non-claims retained

The evidence does not claim DefaultPlugins membership, all-recipe breadth, all-fuel breadth, smoker behavior, blast-furnace behavior, hopper automation, XP behavior, recipe-book synchronization, chunk-unload semantics, fresh live Paper parity, broad furnace parity, broad vanilla parity, broad Minecraft compatibility, public-server safety, or production readiness.
