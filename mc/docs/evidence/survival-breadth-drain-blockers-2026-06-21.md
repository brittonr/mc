# Survival breadth drain blockers — 2026-06-21

## Question
Can the remaining active survival breadth Cairn changes be drained from the current repository state?

## Inspected evidence
- `cairn/changes/*/tasks.md`: each active survival breadth change has the shared contract and checker tasks complete, then blocks at the isolated rail task.
- `docs/evidence/survival-breadth-contracts-2026-06-20.md`: explicitly says each row remains blocked from archive until an isolated rail produces paired Paper/reference and Valence receipts/logs with clean child revision metadata and BLAKE3 manifests.
- `docs/evidence/survival-breadth-drain-baseline-2026-06-21.run.log`: checker self-test and Cairn validation both completed with `exit_status=0`.

## Decision
Leave these changes active. Do not sync/archive them yet: no row currently has reviewable isolated live rail implementation plus paired Paper/reference and Valence receipt/log evidence for the new breadth contract.

Blocked changes:

- `survival-biome-dimension-travel-parity`: needs a new dimension-travel rail and paired evidence for the configured overworld-to-nether transition metrics.
- `survival-container-block-entity-breadth-parity`: needs a new container/block-entity breadth rail and paired evidence for the configured barrel/payload metrics.
- `survival-hunger-health-cycle-parity`: needs an isolated row/evidence path for the health-cycle breadth metrics without reclassifying the existing Bread consumption row.
- `survival-mob-ai-loot-breadth-parity`: needs a new mob AI/loot rail and paired evidence for the configured Zombie/RottenFlesh metrics, not the existing Iron Golem row.
- `survival-redstone-circuit-breadth-parity`: needs a new circuit breadth rail and paired evidence for the configured lever/lamp/repeater tick sequence, not the existing lever/lamp toggle row.
- `survival-sign-editing-live-parity`: needs maintained live server correlation receipts for sign editor open/update; existing deterministic targeted-packet/sign-driver evidence is not paired live Paper/Valence parity.
- `survival-world-multichunk-durability-parity`: needs a new multichunk durability rail and paired evidence for the configured two-chunk post-restart storage metrics.

## Owner
mc-compat implementation owner.

## Next action
Implement one breadth row at a time with functional-core runner/checker updates, Paper fixture support, Valence `survival_compat` support, Stevenarella client probe support, paired live receipts/logs copied under `docs/evidence/`, BLAKE3 manifests, task-evidence gate, Cairn gates, Cairn sync, and archive.
