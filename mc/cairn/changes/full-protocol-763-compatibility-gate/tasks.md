# Tasks

- [x] [serial] Define the bounded `full protocol-763 compatibility aggregate` evidence contract and normalized metric names. r[mc_compatibility.full_protocol_763_compatibility_gate.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `full protocol-763 compatibility aggregate`. r[mc_compatibility.full_protocol_763_compatibility_gate.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [ ] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.full_protocol_763_compatibility_gate.evidence_standard]
- [ ] [depends:evidence_standard] Add `full-protocol-763-compatibility-gate` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.full_protocol_763_compatibility_gate.rail]
  - Detail: Define packet-family completion criteria.
  - Detail: Add aggregate checker fixtures.
  - Detail: Wire broad coverage ledger/current bundle claim blocks.
  - Detail: Promote aggregate only after ledger complete.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.full_protocol_763_compatibility_gate.artifacts]
- [ ] [depends:artifacts] Promote only the `full protocol-763 compatibility aggregate` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.full_protocol_763_compatibility_gate.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.full_protocol_763_compatibility_gate.validation]
