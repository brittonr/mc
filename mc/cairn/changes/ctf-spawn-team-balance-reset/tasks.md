# Tasks

- [x] [serial] Define the bounded `spawn/team balance/resource reset` evidence contract and normalized metric names. r[mc_compatibility.ctf_spawn_team_balance_reset.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `spawn/team balance/resource reset`. r[mc_compatibility.ctf_spawn_team_balance_reset.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [x] [depends:checker] Add the `ctf-spawn-team-balance-reset` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.ctf_spawn_team_balance_reset.rail] Evidence: `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.run.log`, `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.b3`.
  - Detail: Define spawn/team/reset contract.
  - Detail: Add checker fixtures.
  - Detail: Add CTF spawn/team rail.
  - Detail: Promote only configured reset row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.ctf_spawn_team_balance_reset.evidence]
- [ ] [depends:evidence] Promote only the `spawn/team balance/resource reset` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.ctf_spawn_team_balance_reset.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.ctf_spawn_team_balance_reset.validation]
