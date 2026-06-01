# Tasks

- [x] [serial] Define the bounded `projectile travel/collision` evidence contract and normalized metric names. r[mc_compatibility.projectile_travel_collision_parity.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `projectile travel/collision`. r[mc_compatibility.projectile_travel_collision_parity.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [x] [depends:checker] Add the `projectile-travel-collision` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.projectile_travel_collision_parity.rail] Evidence: `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.run.log`, `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.b3`.
  - Detail: Define projectile path and impact metric contract.
  - Detail: Add projectile comparator/checker fixtures.
  - Detail: Add runner/client projectile travel milestones.
  - Detail: Promote only configured weapon/path row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.projectile_travel_collision_parity.evidence]
- [ ] [depends:evidence] Promote only the `projectile travel/collision` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.projectile_travel_collision_parity.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.projectile_travel_collision_parity.validation]
