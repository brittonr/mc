# Evidence manifest refresh automation evidence

## Scope

This evidence covers the `automate-evidence-manifest-refresh` Cairn change. It introduces a repo-owned helper for checking or refreshing reviewable `docs/evidence/*.b3` BLAKE3 manifests. The change does not claim new Minecraft compatibility behavior and does not alter receipt schemas.

## Implementation

- `tools/refresh_evidence_manifests.rs` implements a deterministic manifest parser/planner over explicit manifest text and file digest states.
- The imperative shell discovers `docs/evidence/*.b3`, computes BLAKE3 digests with `b3sum`, rewrites only stale digest fields in `--refresh` mode, and leaves missing-file rows visible as failing diagnostics.
- `flake.nix` exposes the helper as `nix run .#evidence-manifest-refresh -- --check|--refresh`, adds `mc-compat-evidence-manifest-refresh`, and runs the helper inside the existing `mc-compat-evidence-manifests` check.
- `README.md` documents the preferred check/refresh workflow before evidence-manifest and task-evidence gates.

## Positive and negative coverage

The helper self-test exercises unchanged manifests, stale manifests, explicit refresh writes, cascading manifest references, missing-file preservation, malformed rows, outside-root rows, and non-converging self-referential manifests.

`docs/evidence/evidence-manifest-refresh-baseline-2026-06-06.run.log` records the pre-change evidence-manifest/task-evidence baseline with `exit_status=0`.

`docs/evidence/evidence-manifest-refresh-focused-checks-2026-06-06.run.log` records passing package/app, dedicated refresh check, and integrated evidence-manifest checks with `exit_status=0`.

`docs/evidence/evidence-manifest-refresh-cairn-gates-2026-06-06.run.log` records passing Cairn proposal/design/tasks gates and validation for the active change.

`docs/evidence/evidence-manifest-refresh-evidence-checks-2026-06-06.run.log` records passing refresh, evidence-manifest, and task-evidence flake checks with `exit_status=0`.

`docs/evidence/evidence-manifest-refresh-task-evidence-final-2026-06-06.run.log` records the final staged-source task-evidence/evidence-manifest check with `exit_status=0`.

`docs/evidence/evidence-manifest-refresh-sync-2026-06-06.run.log` records Cairn sync execution. The pinned Cairn sync reported no accepted-spec content change, so `docs/evidence/evidence-manifest-refresh-post-sync-checks-2026-06-06.run.log` records the manual accepted-spec merge check, manifest fixpoint check, and Cairn validation with `exit_status=0`.

`docs/evidence/evidence-manifest-refresh-archive-2026-06-06.run.log` records archive execution to `cairn/archive/2026-06-06-automate-evidence-manifest-refresh` and post-archive Cairn validation with `exit_status=0`.

`docs/evidence/evidence-manifest-refresh-post-archive-checks-2026-06-06.run.log` records the final post-archive focused Nix checks, task-evidence/evidence-manifest gates, Cairn validation, and active-change listing with `exit_status=0`.

## Non-claims

- This is evidence of manifest maintenance tooling only.
- It is not live Paper/Valence parity evidence.
- It is not a new compatibility row, public-server-safety claim, production-readiness claim, or receipt-schema change.
