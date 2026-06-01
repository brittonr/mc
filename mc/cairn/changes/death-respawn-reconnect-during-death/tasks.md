# Tasks

- [x] [serial] Define the bounded `reconnect during death` evidence contract and normalized metric names. r[mc_compatibility.death_respawn_reconnect_during_death.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `reconnect during death`. r[mc_compatibility.death_respawn_reconnect_during_death.checker]
- [ ] [depends:checker] Add the `death-respawn-reconnect-during-death` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.death_respawn_reconnect_during_death.rail]
  - Detail: Define death-reconnect state machine.
  - Detail: Add checker negative fixtures.
  - Detail: Add two-session runner rail.
  - Detail: Promote only configured reconnect timing row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.death_respawn_reconnect_during_death.evidence]
- [ ] [depends:evidence] Promote only the `reconnect during death` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.death_respawn_reconnect_during_death.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.death_respawn_reconnect_during_death.validation]
