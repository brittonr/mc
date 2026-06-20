# Tasks

- [x] [serial] Define the failure-bundle schema, fail-only outcome policy, BLAKE3 artifact identity requirements, path policy, and diagnostic-only non-claim boundary. r[mc_compatibility.failure_evidence_bundles.contract]
  Evidence: docs/evidence/failure-evidence-bundles-2026-06-20.run.log; docs/evidence/failure-evidence-bundles-2026-06-20.b3
- [x] [depends:contract] Add a pure failure-bundle validator with positive and negative fixtures for complete bundles, missing artifacts, path escapes, malformed digests, missing nonclaims, and success-labeled bundles. r[mc_compatibility.failure_evidence_bundles.validator]
  Evidence: docs/evidence/failure-evidence-bundles-2026-06-20.run.log; docs/evidence/failure-evidence-bundles-2026-06-20.b3
- [x] [depends:validator] Extend runner failure paths to collect receipt/log/typed-event/stderr metadata and write bounded failure bundles without hiding the original failure exit status. r[mc_compatibility.failure_evidence_bundles.runner]
  Evidence: docs/evidence/failure-evidence-bundles-2026-06-20.run.log; docs/evidence/failure-evidence-bundles-2026-06-20.b3
- [x] [depends:runner] Add documentation for when and how to copy failure bundles into `docs/evidence/` with BLAKE3 manifests for Cairn review. r[mc_compatibility.failure_evidence_bundles.docs]
  Evidence: docs/evidence/failure-evidence-bundles-2026-06-20.run.log; docs/evidence/failure-evidence-bundles-2026-06-20.b3
- [x] [depends:docs] Run failure-bundle fixtures, runner failure-path tests, affected dry-run checks, evidence manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.failure_evidence_bundles.validation]
  Evidence: docs/evidence/failure-evidence-bundles-2026-06-20.run.log; docs/evidence/failure-evidence-bundles-evidence-manifest-2026-06-20.run.log; docs/evidence/failure-evidence-bundles-2026-06-20.b3; docs/evidence/failure-evidence-bundles-evidence-manifest-2026-06-20.b3
