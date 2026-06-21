# Survival breadth drain blockers — 2026-06-21

## Question
Can the remaining active survival breadth Cairn changes be drained from the current repository state?

## Inspected evidence
- `cairn/changes/*/tasks.md`: the remaining active survival breadth changes still have the shared contract and checker tasks complete, then block at the isolated rail task.
- `docs/evidence/survival-breadth-contracts-2026-06-20.md`: each row remains blocked from archive until an isolated rail produces paired Paper/reference and Valence receipts/logs with clean child revision metadata and BLAKE3 manifests.
- `docs/evidence/survival-remaining-breadth-baseline-checker-final-2026-06-21.run.log`: the shared breadth checker self-test completed with `exit_status=0` after two command-shape baseline failures (`permission denied` direct script execution and a shared Cargo.lock version error without the prior isolated target-dir invocation).
- `docs/evidence/survival-remaining-breadth-baseline-runner-2026-06-21.run.log`: the runner unit-test baseline completed with `exit_status=0`.
- `docs/evidence/survival-remaining-breadth-drain-assessment-2026-06-21.run.log`: current Cairn validation and proposal/design/tasks gates completed with `exit_status=0` for the remaining active changes.
- Child-repo status before any new child edits already showed modified `stevenarella/src/server/mod.rs` and `valence/examples/survival_compat.rs`; these appear to be hunger-health-cycle rail work and must not be overwritten while draining the remaining rows.

## Decision
Leave the remaining changes active. Do not sync/archive them yet: no remaining row currently has reviewable isolated live rail implementation plus paired Paper/reference and Valence receipt/log evidence for the new breadth contract, and child-repo revisions are not clean enough for new live receipt metadata.

Blocked changes:

- `survival-biome-dimension-travel-parity`: needs a new dimension-travel rail and paired evidence for the configured overworld-to-nether transition metrics; the existing biome/dimension state row only proves a join-state observation.
- `survival-container-block-entity-breadth-parity`: needs a new container/block-entity breadth rail and paired evidence for the configured Barrel transfer/payload metrics; existing chest persistence and sign block-entity rows are not this row.
- `survival-mob-ai-loot-breadth-parity`: needs contract/spec alignment before implementation because the proposal/spec delta names hostile/passive breadth while the shared 2026-06-20 contract/checker fixture names one configured `Zombie`/`RottenFlesh` row. After alignment it still needs a new rail and paired evidence distinct from the existing Iron Golem row.
- `survival-redstone-circuit-breadth-parity`: needs a new circuit rail and paired evidence for the configured lever/lamp/repeater tick sequence; the existing lever/lamp toggle row does not provide the required tick-sequence metrics.
- `survival-sign-editing-live-parity`: needs maintained live server-correlation receipts for sign editor open/update; existing deterministic targeted-packet/sign-driver evidence is not paired live Paper/Valence parity.
- `survival-world-multichunk-durability-parity`: needs a new multichunk durability rail and paired evidence for the configured two-chunk post-restart storage metrics; existing single-block persistence/crash-recovery rows are not sufficient.

## Owner
mc-compat implementation owner.

## Next action
First decide whether to commit or otherwise preserve the existing child-repo hunger-health changes so new receipt metadata can be clean. Then implement one remaining breadth row at a time with functional-core runner/checker updates, Paper fixture support, Valence `survival_compat` support, Stevenarella client probe support, paired live receipts/logs copied under `docs/evidence/`, BLAKE3 manifests, task-evidence gate, Cairn gates, Cairn sync, and archive.
