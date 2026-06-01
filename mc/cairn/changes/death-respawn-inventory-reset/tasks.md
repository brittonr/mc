# Tasks

- [x] [serial] Define the bounded `death inventory reset` evidence contract and normalized metric names. r[mc_compatibility.death_respawn_inventory_reset.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `death inventory reset`. r[mc_compatibility.death_respawn_inventory_reset.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [ ] [depends:checker] Add the `death-respawn-inventory-reset` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.death_respawn_inventory_reset.rail]
  - Detail: Define death inventory policy contract.
  - Detail: Add lifecycle checker fixtures.
  - Detail: Add runner/client death inventory rail.
  - Detail: Promote only configured policy row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.death_respawn_inventory_reset.evidence]
- [ ] [depends:evidence] Promote only the `death inventory reset` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.death_respawn_inventory_reset.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.death_respawn_inventory_reset.validation]
