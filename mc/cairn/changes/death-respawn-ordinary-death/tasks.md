# Tasks

- [x] [serial] Define the bounded `ordinary death/respawn` evidence contract and normalized metric names. r[mc_compatibility.death_respawn_ordinary_death.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `ordinary death/respawn`. r[mc_compatibility.death_respawn_ordinary_death.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [x] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.death_respawn_ordinary_death.evidence_standard] Evidence: `docs/evidence/mc-compat-row-evidence-standards-2026-06-01.run.log`, `docs/evidence/mc-compat-row-evidence-standards-2026-06-01.b3`.
- [x] [depends:evidence_standard] Add `death-respawn-ordinary-death` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.death_respawn_ordinary_death.rail] Evidence: `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.run.log`, `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.b3`.
  - Detail: Define ordinary-death lifecycle contract.
  - Detail: Add checker positive/negative fixtures.
  - Detail: Add runner/client ordinary-death rail.
  - Detail: Update death/respawn lifecycle doc only for this row.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.death_respawn_ordinary_death.artifacts]
- [ ] [depends:artifacts] Promote only the `ordinary death/respawn` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.death_respawn_ordinary_death.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.death_respawn_ordinary_death.validation]
