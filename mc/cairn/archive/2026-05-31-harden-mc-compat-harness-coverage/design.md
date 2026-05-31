# Design: Harden mc-compat harness coverage gates

## Context

`tools/check_cairn_task_evidence.rs` is the active-task gate that prevents checked Cairn tasks from relying only on prose. It already requires an Evidence line, a copied `docs/evidence/` artifact, a `.run.log`, and BLAKE3 evidence. The hardening keeps that gate fast and deterministic while making the evidence chain more reviewable.

## Decisions

### 1. Keep the gate pure-core plus filesystem shell

**Choice:** Read active task files and evidence catalog in the shell, then pass immutable task/evidence structures into pure validation functions.

**Rationale:** Self-test fixtures can exercise fail-closed branches without touching the real filesystem, while the repo check still validates real active tasks and evidence files.

### 2. Check run-log contents, not just paths

**Choice:** A completed task's cited `.run.log` must include at least one `exit_status=` line, and every such line must resolve to `0`.

**Rationale:** Review logs without explicit command status are ambiguous. Multiple-command logs remain supported because each command can record its own `<name>_exit_status=0` line.

### 3. Reject non-reviewable artifact path references

**Choice:** Completed-task evidence rejects path-like references rooted at `target/` or nested child checkouts such as `stevenarella/`, `valence/`, `hyperion/`, and `Leafish/`.

**Rationale:** Parent Nix checks and reviewers cannot rely on ignored build outputs or nested repos unless the exact bytes are copied under parent `docs/evidence/`.

### 4. Pair run logs with cited manifests

**Choice:** When a completed task cites `.b3` sidecars instead of an inline digest, at least one cited sidecar must contain each cited `.run.log` path.

**Rationale:** This catches stale or unrelated manifests that exist but do not cover the verification output named by the task.

## Risks

- Older active tasks may fail if they cite logs without explicit status lines; this is intended and should be fixed by recording a proper validation log.
- The path-prefix rejection is scoped to completed-task text to avoid blocking ordinary proposals/design notes.
