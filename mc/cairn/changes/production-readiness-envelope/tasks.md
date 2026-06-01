# Tasks

- [x] [serial] Define the bounded `production readiness aggregate` evidence contract and normalized metric names. r[mc_compatibility.production_readiness_envelope.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `production readiness aggregate`. r[mc_compatibility.production_readiness_envelope.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [ ] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.production_readiness_envelope.evidence_standard]
- [ ] [depends:evidence_standard] Add `production-readiness-envelope` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.production_readiness_envelope.rail]
  - Detail: Define production readiness row set.
  - Detail: Add aggregate checker fixtures.
  - Detail: Require redacted evidence and human checkpoints.
  - Detail: Block production wording until envelope complete.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.production_readiness_envelope.artifacts]
- [ ] [depends:artifacts] Promote only the `production readiness aggregate` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.production_readiness_envelope.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.production_readiness_envelope.validation]
