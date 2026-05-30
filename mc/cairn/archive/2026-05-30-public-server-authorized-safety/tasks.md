# Tasks

- [x] [serial] Define the bounded `public-server safety` evidence contract and normalized metric names. r[mc_compatibility.public_server_authorized_safety.contract]
  - Evidence: `docs/evidence/protocol-763-public-server-authorized-safety-contract-2026-05-30.md`; `docs/evidence/protocol-763-public-server-authorized-safety-checker-2026-05-30.run.log`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `public-server safety`. r[mc_compatibility.public_server_authorized_safety.checker]
  - Evidence: `tools/check_public_server_authorized_safety.rs`; `docs/evidence/protocol-763-public-server-authorized-safety-checker-2026-05-30.run.log`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`.
- [x] [depends:checker] Add the `public-server-authorized-safety` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.public_server_authorized_safety.rail]
  - Detail: Define authorization evidence contract.
  - Detail: Add preflight rejection tests.
  - Detail: Add redacted telemetry receipt.
  - Detail: Promote only authorized bounded envelope.
  - Evidence: `docs/evidence/protocol-763-public-server-authorized-safety-dry-run-2026-05-30.run.log`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.receipt.json`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`.
- [x] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.public_server_authorized_safety.evidence]
  - Evidence: `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.md`; `docs/evidence/protocol-763-public-server-authorized-safety-checker-2026-05-30.run.log`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`.
- [x] [depends:evidence] Promote only the `public-server safety` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.public_server_authorized_safety.matrix]
  - Evidence: `docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md`; `docs/evidence/protocol-763-public-server-authorized-safety-checker-2026-05-30.run.log`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`.
- [x] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.public_server_authorized_safety.validation]
  - Evidence: `docs/evidence/protocol-763-public-server-authorized-safety-validation-2026-05-30.run.log`; `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`.
