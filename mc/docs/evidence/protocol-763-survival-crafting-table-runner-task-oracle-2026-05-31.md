# Survival crafting table runner/client rail task oracle — 2026-05-31

## Question
Can the in-progress `runner` task in `cairn/changes/survival-crafting-table-parity/tasks.md` be marked complete after adding the Stevenarella client probe rail?

## Inspected evidence
- `docs/evidence/protocol-763-survival-crafting-table-runner-baseline-2026-05-31.run.log` records pre-change focused server tests and the existing runner scenario test at parent revision `468069f` and Stevenarella child revision `0583455`, both with `exit_status=0`.
- `stevenarella` child commit `aacc7b4` (`add survival crafting client probe rail`) adds the `MC_COMPAT_SURVIVAL_CRAFTING_PROBE` client path for the configured table at `4,64,0`, input slots `1` and `4`, result slot `0`, inventory slot `36`, `OakPlanks` input, `Stick` result count `4`, and recipe label `minecraft:stick`.
- The child commit logs the runner-required client milestones: table open, input A/B sent, result seen, result collected, and inventory updated. It keeps fixture/server-side milestone generation out of this task.
- `docs/evidence/protocol-763-survival-crafting-table-runner-source-2026-05-31.patch` records the exact child source diff.
- `docs/evidence/protocol-763-survival-crafting-table-runner-2026-05-31.run.log` records `cargo fmt --check`, focused Stevenarella server tests, full Stevenarella library tests, `cargo check --bin stevenarella`, the runner crafting scenario test, and the Valence dry-run wrapper with `exit_status=0`.
- `docs/evidence/protocol-763-survival-crafting-table-runner-2026-05-31.b3` hashes the baseline log, validation log, and source patch.
- `docs/evidence/protocol-763-survival-crafting-table-runner-source-2026-05-31.b3` hashes the source patch.

## Finding
The runner/client rail is now complete: parent runner scenario selection and milestone expectations already existed, and the Stevenarella child repo now has the matching client probe that can emit the required client-side crafting milestones without changing existing survival, chest, inventory, CTF, combat, or projectile scenario semantics. This evidence does not claim Paper/Valence fixture instrumentation, paired receipts, matrix promotion, live compatibility, full crafting coverage, recipe-book behavior, shift-click behavior, full survival compatibility, or production readiness.

## Owner
Britton Robitzsch, mc compatibility owner.

## Decision
Mark the `runner` task complete. Leave `fixtures`, `receipts`, `matrix`, and final `validation` tasks open.
