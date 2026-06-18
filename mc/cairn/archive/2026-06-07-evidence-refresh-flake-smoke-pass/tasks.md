# Tasks

- [x] [serial] Record the bounded smoke/evidence refresh contract: selected checks, dry-runs, command scope, runtime limits, and non-claims. r[mc_compatibility.evidence_refresh_flake_smoke.contract]
  - Evidence: `docs/evidence/evidence-refresh-flake-smoke-contract-2026-06-07.md`.
- [x] [depends:contract] Run non-mutating baseline Cairn validation/gates, targeted packet checks, scenario manifest checks, and selected dry-run app checks. r[mc_compatibility.evidence_refresh_flake_smoke.baseline]
  - Evidence: `docs/evidence/evidence-refresh-flake-smoke-baseline-2026-06-07.run.log`.
- [x] [depends:baseline] Promote reviewable smoke logs under `docs/evidence/`, including blocker notes for any fail-closed smoke. r[mc_compatibility.evidence_refresh_flake_smoke.logs]
  - Evidence: `docs/evidence/evidence-refresh-flake-smoke-baseline-2026-06-07.run.log` contains `exit_status=0`; no fail-closed blocker note was required.
- [x] [depends:logs] Refresh BLAKE3 manifests only for tracked evidence files changed by the smoke pass and rerun manifest checks to a deterministic fixpoint. r[mc_compatibility.evidence_refresh_flake_smoke.manifests]
  - Evidence: `docs/evidence/evidence-refresh-flake-smoke-manifest-refresh-2026-06-07.run.log` and updated BLAKE3 manifests under `docs/evidence/`.
- [x] [depends:manifests] Run task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation without promoting compatibility rows. r[mc_compatibility.evidence_refresh_flake_smoke.validation]
  - Evidence: `docs/evidence/evidence-refresh-flake-smoke-closeout-2026-06-07.run.log` and `docs/evidence/evidence-refresh-flake-smoke-2026-06-07.b3`.
