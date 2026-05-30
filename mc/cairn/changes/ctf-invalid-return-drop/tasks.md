# Tasks

- [ ] [serial] Define the bounded `invalid flag return/drop` evidence contract and normalized metric names. r[mc_compatibility.ctf_invalid_return_drop.contract]
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `invalid flag return/drop`. r[mc_compatibility.ctf_invalid_return_drop.checker]
- [ ] [depends:checker] Add the `ctf-invalid-return-drop` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.ctf_invalid_return_drop.rail]
  - Detail: Define invalid return/drop contract.
  - Detail: Add checker positive/negative fixtures.
  - Detail: Add invalid return/drop runner rail.
  - Detail: Promote only bounded rule row.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.ctf_invalid_return_drop.evidence]
- [ ] [depends:evidence] Promote only the `invalid flag return/drop` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.ctf_invalid_return_drop.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.ctf_invalid_return_drop.validation]
