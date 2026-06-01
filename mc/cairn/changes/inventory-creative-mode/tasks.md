# Tasks

- [x] [serial] Define the bounded `creative-mode inventory` evidence contract and normalized metric names. r[mc_compatibility.inventory_creative_mode.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `creative-mode inventory`. r[mc_compatibility.inventory_creative_mode.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [x] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.inventory_creative_mode.evidence_standard] Evidence: `docs/evidence/mc-compat-row-evidence-standards-2026-06-01.run.log`, `docs/evidence/mc-compat-row-evidence-standards-2026-06-01.b3`.
- [x] [depends:evidence_standard] Add `inventory-creative-mode` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.inventory_creative_mode.rail] Evidence: `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.run.log`, `docs/evidence/mc-compat-row-fixture-rail-2026-06-01.b3`.
  - Detail: Define creative permission contract.
  - Detail: Add checker negative fixtures.
  - Detail: Add creative fixture/probe.
  - Detail: Promote only configured creative action.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.inventory_creative_mode.artifacts]
- [ ] [depends:artifacts] Promote only the `creative-mode inventory` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.inventory_creative_mode.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.inventory_creative_mode.validation]
