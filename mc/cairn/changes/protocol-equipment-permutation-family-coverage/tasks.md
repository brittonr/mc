# Tasks

- [x] [serial] Define the bounded `equipment permutation packet family` evidence contract and normalized metric names. r[mc_compatibility.protocol_equipment_permutation_family_coverage.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `equipment permutation packet family`. r[mc_compatibility.protocol_equipment_permutation_family_coverage.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [x] [depends:checker] Add the `protocol-equipment-permutation-family` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.protocol_equipment_permutation_family_coverage.rail] Evidence: `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.run.log`, `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.b3`.
  - Detail: Select equipment permutation subset.
  - Detail: Add parser fixture coverage.
  - Detail: Add observer live evidence.
  - Detail: Update protocol ledger.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.protocol_equipment_permutation_family_coverage.evidence]
- [ ] [depends:evidence] Promote only the `equipment permutation packet family` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.protocol_equipment_permutation_family_coverage.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.protocol_equipment_permutation_family_coverage.validation]
