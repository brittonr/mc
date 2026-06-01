# Tasks

- [x] [serial] Define the bounded `simultaneous pickup/capture race` evidence contract and normalized metric names. r[mc_compatibility.ctf_simultaneous_pickup_capture_race.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `simultaneous pickup/capture race`. r[mc_compatibility.ctf_simultaneous_pickup_capture_race.checker]
- [ ] [depends:checker] Add the `ctf-simultaneous-pickup-capture-race` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.ctf_simultaneous_pickup_capture_race.rail]
  - Detail: Define race oracle contract.
  - Detail: Add multi-client checker fixtures.
  - Detail: Add deterministic race runner rail.
  - Detail: Promote only one race-window row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.ctf_simultaneous_pickup_capture_race.evidence]
- [ ] [depends:evidence] Promote only the `simultaneous pickup/capture race` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.ctf_simultaneous_pickup_capture_race.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.ctf_simultaneous_pickup_capture_race.validation]
