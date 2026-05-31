# Proposal: Harden mc-compat harness coverage gates

## Why

The compatibility harness already records Cairn task evidence and BLAKE3 manifests, but review history shows repeated weak spots: run logs without explicit exit status, task evidence that mentions transient `target/` artifacts, child-repo paths that are not copied into parent `docs/evidence/`, and `.b3` sidecars that do not actually cover cited run logs. These gaps let tasks look complete even when reviewers cannot replay the evidence chain from the parent repo.

## What Changes

- Harden the active Cairn task-evidence gate so completed tasks must cite run logs with explicit successful exit-status lines.
- Reject completed-task evidence that cites transient `target/` artifacts or nested child-repo paths as review-critical artifacts instead of copied parent `docs/evidence/` files.
- Require cited `.b3` manifests to cover cited `.run.log` files unless the task uses an inline BLAKE3 digest.
- Add positive and negative self-test fixtures for each fail-closed branch and record validation output under `docs/evidence/`.

## Impact

- **Files**: `tools/check_cairn_task_evidence.rs`, Cairn change artifacts, and validation logs/manifests under `docs/evidence/`.
- **Validation**: checker self-tests, active task-evidence gate, evidence manifest check, Cairn proposal/tasks gates, and Cairn validation.
- **Non-claims**: this does not add new gameplay/protocol compatibility coverage, live MCP driving, frame capture, or full evidence completeness for archived historical tasks; it hardens active task closeout going forward.
