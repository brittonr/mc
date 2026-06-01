# Tasks

- [x] [serial] Define the bounded `repeated death safety` evidence contract and normalized metric names. r[mc_compatibility.death_respawn_repeated_death_safety.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `repeated death safety`. r[mc_compatibility.death_respawn_repeated_death_safety.checker]
- [ ] [depends:checker] Add the `death-respawn-repeated-death-safety` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.death_respawn_repeated_death_safety.rail]
  - Detail: Define finite death-cycle contract.
  - Detail: Add checker fixtures.
  - Detail: Add repeated death runner rail.
  - Detail: Promote only configured cycle count row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.death_respawn_repeated_death_safety.evidence]
- [ ] [depends:evidence] Promote only the `repeated death safety` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.death_respawn_repeated_death_safety.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.death_respawn_repeated_death_safety.validation]
