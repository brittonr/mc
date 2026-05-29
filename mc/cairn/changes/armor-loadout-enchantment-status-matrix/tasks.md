# Tasks

- [x] [serial] Define the bounded `armor/enchantment/status matrix` evidence contract and normalized metric names. r[mc_compatibility.armor_loadout_enchantment_status_matrix.contract]
  - Evidence: `docs/evidence/protocol-763-armor-loadout-enchantment-status-contract-2026-05-29.md` defines bounded candidate rows, normalized metrics, promotion requirements, reject cases, and non-claims; verification output is in `docs/evidence/protocol-763-armor-loadout-enchantment-status-contract-2026-05-29.run.log`; BLAKE3 manifest `docs/evidence/protocol-763-armor-loadout-enchantment-status-contract-2026-05-29.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `armor/enchantment/status matrix`. r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker]
- [ ] [depends:checker] Add the `armor-loadout-enchantment-status-matrix` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.armor_loadout_enchantment_status_matrix.rail]
  - Detail: Define declarative armor/modifier row schema.
  - Detail: Add matrix checker fixtures.
  - Detail: Add first expanded live rows.
  - Detail: Update residual combat catalog and current bundle labels.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.armor_loadout_enchantment_status_matrix.evidence]
- [ ] [depends:evidence] Promote only the `armor/enchantment/status matrix` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.armor_loadout_enchantment_status_matrix.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.armor_loadout_enchantment_status_matrix.validation]
