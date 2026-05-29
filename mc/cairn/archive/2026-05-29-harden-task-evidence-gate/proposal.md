## Why

Done-review has repeatedly caught completed Cairn tasks that claimed verification while the durable repo evidence was incomplete: target-only receipts, missing BLAKE3 sidecars, absent command output, or broad completion claims without copied artifacts. Those misses are easy to prevent with a deterministic local gate before archive.

## What Changes

- Add a Rust task-evidence checker for active `cairn/changes/*/tasks.md` files.
- Fail checked tasks unless they cite copied `docs/evidence/` artifacts, verification command output, and a BLAKE3 manifest or inline digest.
- Wire the checker into flake checks and the maintained aggregate so future closeout runs catch missing evidence before review.
- Document the task closeout contract in README.

## Impact

- **Files**: `tools/check_cairn_task_evidence.rs`, `flake.nix`, `README.md`, `cairn/specs/mc-compatibility/spec.md`, `docs/evidence/*task-evidence*`.
- **Testing**: checker self-tests, live active-change scan, flake check, Cairn validation/gates, evidence manifest validation.
