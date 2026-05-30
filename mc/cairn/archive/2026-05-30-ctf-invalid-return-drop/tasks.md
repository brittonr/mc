# Tasks

- [x] [serial] Define the bounded `invalid flag return/drop` evidence contract and normalized metric names. r[mc_compatibility.ctf_invalid_return_drop.contract]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-return-drop-contract-2026-05-30.md`, `docs/evidence/protocol-763-ctf-invalid-return-drop-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `invalid flag return/drop`. r[mc_compatibility.ctf_invalid_return_drop.checker]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.record`, `docs/evidence/protocol-763-ctf-invalid-return-drop-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`.
- [x] [depends:checker] Add the `ctf-invalid-return-drop` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.ctf_invalid_return_drop.rail]
  - Detail: Define invalid return/drop contract.
  - Detail: Add checker positive/negative fixtures.
  - Detail: Add invalid return/drop runner rail.
  - Detail: Promote only bounded rule row.
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.client.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.server.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`.
- [x] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.ctf_invalid_return_drop.evidence]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.md`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`.
- [x] [depends:evidence] Promote only the `invalid flag return/drop` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.ctf_invalid_return_drop.matrix]
  - Evidence: `docs/evidence/protocol-763-acceptance-matrix.md`, `docs/evidence/protocol-763-current-evidence-bundle.md`, `docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md`, `docs/evidence/protocol-763-ctf-invalid-return-drop-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`.
- [x] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.ctf_invalid_return_drop.validation]
  - Evidence: `docs/evidence/protocol-763-ctf-invalid-return-drop-validation-2026-05-30.run.log`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.record`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.b3`.
