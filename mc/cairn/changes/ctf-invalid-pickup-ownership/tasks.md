# Tasks

- [x] [serial] Define the bounded `invalid flag pickup/ownership` evidence contract and normalized metric names. r[mc_compatibility.ctf_invalid_pickup_ownership.contract]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-contract-2026-05-30.md`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `invalid flag pickup/ownership`. r[mc_compatibility.ctf_invalid_pickup_ownership.checker]
  - Evidence: `tools/check_ctf_invalid_pickup_ownership.rs`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.record`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`.
- [x] [depends:checker] Add the `ctf-invalid-pickup-ownership` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.ctf_invalid_pickup_ownership.rail]
  - Detail: Define invalid pickup ownership contract.
  - Detail: Add negative checker fixtures.
  - Detail: Add runner/client invalid pickup rail.
  - Detail: Update CTF rule ledger only for this row.
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.receipt.json`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.client.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.server.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`.
- [x] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.ctf_invalid_pickup_ownership.evidence]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.md`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.receipt.json`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`.
- [x] [depends:evidence] Promote only the `invalid flag pickup/ownership` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.ctf_invalid_pickup_ownership.matrix]
  - Evidence: `docs/evidence/protocol-763-acceptance-matrix.md`, `docs/evidence/protocol-763-current-evidence-bundle.md`, `docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`.
- [x] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.ctf_invalid_pickup_ownership.validation]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.record`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.b3`.
