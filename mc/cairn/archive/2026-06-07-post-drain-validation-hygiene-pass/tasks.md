# Tasks

- [x] [serial] Record the hygiene scope and non-claim contract before running or refreshing evidence. r[mc_compatibility.post_drain_validation_hygiene.contract]
  - Evidence: `docs/evidence/post-drain-validation-hygiene-scope-2026-06-07.md`, `docs/evidence/post-drain-validation-hygiene-contract-2026-06-07.run.log`, and `docs/evidence/post-drain-validation-hygiene-2026-06-07.b3`.
- [x] [depends:contract] Run the repo-pinned Cairn validation/gate baseline and classify diagnostics without mutating evidence first. r[mc_compatibility.post_drain_validation_hygiene.baseline]
  - Evidence: `docs/evidence/post-drain-validation-hygiene-baseline-local-retry-2026-06-07.run.log` and `docs/evidence/post-drain-validation-hygiene-2026-06-07.b3`.
- [x] [depends:baseline] Refresh or repair only deterministic metadata drift, such as stale tracked BLAKE3 rows or stale drain-state notes. r[mc_compatibility.post_drain_validation_hygiene.remediation]
  - Evidence: `docs/evidence/post-drain-validation-hygiene-remediation-2026-06-07.run.log` and `docs/evidence/post-drain-validation-hygiene-2026-06-07.b3`.
- [x] [depends:remediation] Record reviewable hygiene evidence under `docs/evidence/`, including positive clean-check results and negative/blocker diagnostics when checks fail closed. r[mc_compatibility.post_drain_validation_hygiene.evidence]
  - Evidence: `docs/evidence/post-drain-validation-hygiene-evidence-index-2026-06-07.run.log` and `docs/evidence/post-drain-validation-hygiene-2026-06-07.b3`.
- [x] [depends:evidence] Rerun Cairn validation/gates, evidence-manifest checks, task-evidence checks, and relevant matrix/current-bundle checks after remediation. r[mc_compatibility.post_drain_validation_hygiene.validation]
  - Evidence: `docs/evidence/post-drain-validation-hygiene-validation-precloseout-2026-06-07.run.log` and `docs/evidence/post-drain-validation-hygiene-2026-06-07.b3`.
