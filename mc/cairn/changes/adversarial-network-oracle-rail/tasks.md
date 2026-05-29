# Tasks

- [x] [serial] Define the bounded `adversarial-network safety` evidence contract and normalized metric names. r[mc_compatibility.adversarial_network_oracle_rail.contract]
  - Evidence: `docs/evidence/protocol-763-adversarial-network-oracle-contract-2026-05-29.md` defines the fixture-only threat model, normalized metrics, checker reject cases, checkpoint template, and non-claims; verification output is in `docs/evidence/protocol-763-adversarial-network-oracle-contract-2026-05-29.run.log`; BLAKE3 manifest `docs/evidence/protocol-763-adversarial-network-oracle-contract-2026-05-29.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `adversarial-network safety`. r[mc_compatibility.adversarial_network_oracle_rail.checker]
  - Evidence: `tools/check_adversarial_network_oracle.rs` adds a pure key/value oracle-record validator with positive and fail-closed negative fixtures; `flake.nix` adds `mc-compat-adversarial-network-oracle`; verification output is in `docs/evidence/protocol-763-adversarial-network-oracle-checker-2026-05-29.run.log`; BLAKE3 manifest `docs/evidence/protocol-763-adversarial-network-oracle-checker-2026-05-29.b3`.
- [ ] [depends:checker] Add the `adversarial-network-oracle` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.adversarial_network_oracle_rail.rail]
  - Detail: Define adversarial oracle checkpoint template.
  - Detail: Add fail-closed preflight tests.
  - Detail: Add bounded mutation telemetry contract.
  - Detail: Promote only approved model row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.adversarial_network_oracle_rail.evidence]
- [ ] [depends:evidence] Promote only the `adversarial-network safety` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.adversarial_network_oracle_rail.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.adversarial_network_oracle_rail.validation]
