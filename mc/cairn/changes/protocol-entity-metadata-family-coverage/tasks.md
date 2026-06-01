# Tasks

- [x] [serial] Define the bounded `entity metadata packet family` evidence contract and normalized metric names. r[mc_compatibility.protocol_entity_metadata_family_coverage.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `entity metadata packet family`. r[mc_compatibility.protocol_entity_metadata_family_coverage.checker]
- [ ] [depends:checker] Add the `protocol-entity-metadata-family` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.protocol_entity_metadata_family_coverage.rail]
  - Detail: Select metadata subset from packet inventory.
  - Detail: Add parser fixture positives/negatives.
  - Detail: Add live metadata receipt.
  - Detail: Update broad coverage ledger only for selected rows.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.protocol_entity_metadata_family_coverage.evidence]
- [ ] [depends:evidence] Promote only the `entity metadata packet family` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.protocol_entity_metadata_family_coverage.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.protocol_entity_metadata_family_coverage.validation]
