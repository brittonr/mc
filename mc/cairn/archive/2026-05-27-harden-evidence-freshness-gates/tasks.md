# Tasks

- [x] [serial] Harden cross-file evidence freshness checks. r[mc_compatibility.harden_evidence_freshness_gates.cross_file_freshness]
- [x] [serial] Add positive and negative stale-artifact fixtures. r[mc_compatibility.harden_evidence_freshness_gates.freshness_fixtures]
- [x] [serial] Require review-critical artifacts under `docs/evidence/`. r[mc_compatibility.harden_evidence_freshness_gates.reviewable_artifacts]
- [x] [serial] Document and run the promotion gate. r[mc_compatibility.harden_evidence_freshness_gates.promotion_gate]

## Progress

- `tools/check_acceptance_matrix.py` now verifies evidence doc paths exist and cite the row BLAKE3, docs/evidence receipt rows have matching manifest entries, and non-historical `target/` receipt rows fail closed.
- `tools/check_current_evidence_bundle.py` now has self-test fixtures for missing matrix rows, missing bundle rows, hash mismatches, and missing required gate text.
- `tools/check_evidence_manifests.py` self-tests now cover missing run-log artifacts in addition to stale markers, missing files, and stale hashes.
- Reviewable matrix receipt copies and BLAKE3 manifest were added under `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.*`.
- The current bundle documents the freshness promotion gate and the RED/BLUE scoring soak historical exception.
- Gate evidence passed in `docs/evidence/protocol-763-evidence-freshness-gate-2026-05-27.run.log` with BLAKE3 sidecar `docs/evidence/protocol-763-evidence-freshness-gate-2026-05-27.b3`.
