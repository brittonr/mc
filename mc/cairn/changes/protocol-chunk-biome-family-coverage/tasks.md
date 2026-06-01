# Tasks

- [x] [serial] Define the bounded `chunk/biome packet family` evidence contract and normalized metric names. r[mc_compatibility.protocol_chunk_biome_family_coverage.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `chunk/biome packet family`. r[mc_compatibility.protocol_chunk_biome_family_coverage.checker]
- [ ] [depends:checker] Add the `protocol-chunk-biome-family` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.protocol_chunk_biome_family_coverage.rail]
  - Detail: Select chunk/biome subset.
  - Detail: Add parser fixtures.
  - Detail: Add live environment receipt.
  - Detail: Update broad ledger with non-claims.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.protocol_chunk_biome_family_coverage.evidence]
- [ ] [depends:evidence] Promote only the `chunk/biome packet family` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.protocol_chunk_biome_family_coverage.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.protocol_chunk_biome_family_coverage.validation]
