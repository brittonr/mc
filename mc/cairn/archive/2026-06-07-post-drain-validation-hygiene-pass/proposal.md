## Why

The active Cairn queue was drained on 2026-06-07. Before choosing the next implementation-heavy rail, the workspace needs a bounded hygiene pass that proves the accepted specs, policy, evidence manifests, task-evidence rules, and drain-state notes still agree.

## What Changes

- Run the repo-pinned Cairn validation and evidence gates against the current workspace state.
- Record diagnostics for stale BLAKE3 manifests, missing `exit_status=0` evidence, task/evidence path drift, and policy/schema compatibility.
- Refresh or repair review metadata only when the diagnostics identify deterministic drift.
- Preserve existing compatibility claims and non-claims; this change does not add gameplay, protocol, public-server, production-readiness, or live-parity coverage.

## Impact

- **Files**: `docs/evidence/**`, `cairn/changes/.drain-state.md`, generated policy/evidence manifests when drift is found.
- **Testing**: Cairn validate/gates, evidence-manifest checks, task-evidence checks, targeted matrix/bundle sanity checks, and reviewable run logs with explicit successful exit statuses.
