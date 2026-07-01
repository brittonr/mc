# Furnace smelting Valence shell contract evidence

## Change

`add-furnace-smelting-valence-shell-contract` adds a reviewable Valence Bevy/ECS shell contract for the selected-row furnace-smelting chain without adding runtime systems, schedule wiring, or DefaultPlugins membership.

## Promoted artifacts

- Shell contract: `docs/furnace-smelting-valence-shell-contract.md`.
- Inventory note: `docs/evidence/furnace-smelting-valence-shell-contract-inventory-2026-07-01.md`.
- Focused validator: `tools/check_furnace_smelting_valence_shell_contract.rs`.
- Updated predecessor docs: `docs/furnace-smelting-behavior-card.md`, `docs/furnace-smelting-selected-row-core.md`, `docs/furnace-smelting-selected-row-data-fixture.md`, and `docs/furnace-smelting-selected-row-receipt-handoff.md`.

## Validation logs

- Baseline fixture/core/receipt checks: `docs/evidence/furnace-smelting-valence-shell-contract-baseline-2026-07-01.run.log` (`overall_exit_status=0`).
- Contract validator positive and negative tests plus current-document check: `docs/evidence/furnace-smelting-valence-shell-contract-focused-validation-2026-07-01.run.log` (`overall_exit_status=0`).
- Rust formatting for the focused validator: `docs/evidence/furnace-smelting-valence-shell-contract-rustfmt-2026-07-01.run.log` (`exit_status=0`).
- Post-doc predecessor checks and schedule hygiene source check: `docs/evidence/furnace-smelting-valence-shell-contract-post-docs-validation-2026-07-01.run.log` (`overall_exit_status=0`).
- Task-evidence precloseout gate for implementation tasks: `docs/evidence/furnace-smelting-valence-shell-contract-task-evidence-precloseout-2026-07-01.run.log` (`exit_status=0`).
- Accepted-spec sync dry-run: `docs/evidence/furnace-smelting-valence-shell-contract-sync-dry-run-2026-07-01.run.log` (`exit_status=0`, `blocked=false`).
- Accepted-spec sync execute and requirement-ID verification: `docs/evidence/furnace-smelting-valence-shell-contract-sync-execute-2026-07-01.run.log` (`overall_exit_status=0`).
- Cairn proposal/design/tasks gates and validation after sync: `docs/evidence/furnace-smelting-valence-shell-contract-cairn-gates-2026-07-01.run.log` (`overall_exit_status=0`).
- Evidence manifest refresh/check after accepted-spec sync: `docs/evidence/furnace-smelting-valence-shell-contract-evidence-manifest-refresh-2026-07-01.run.log` (`overall_exit_status=0`).
- Archive dry-run: `docs/evidence/furnace-smelting-valence-shell-contract-archive-dry-run-2026-07-01.run.log` (`exit_status=0`, `blocked=false`).
- Archive execute and post-archive validation: `docs/evidence/furnace-smelting-valence-shell-contract-archive-execute-2026-07-01.run.log` (`overall_exit_status=0`, remaining active changes `[]`).

## Non-claims

This evidence does not claim Valence runtime integration, a Bevy system, schedule registration, DefaultPlugins membership, all-recipe breadth, all-fuel breadth, smoker behavior, blast-furnace behavior, hopper behavior, XP, recipe-book synchronization, chunk-unload semantics, broad furnace parity, broad vanilla parity, broad Minecraft compatibility, public-server safety, or production readiness.
