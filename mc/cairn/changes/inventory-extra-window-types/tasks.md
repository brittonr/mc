# Tasks

- [x] [serial] Define the bounded `extra inventory window types` evidence contract and normalized metric names. r[mc_compatibility.inventory_extra_window_types.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `extra inventory window types`. r[mc_compatibility.inventory_extra_window_types.checker]
- [ ] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.inventory_extra_window_types.evidence_standard]
- [ ] [depends:evidence_standard] Add `inventory-extra-window-types` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.inventory_extra_window_types.rail]
  - Detail: Select next window type.
  - Detail: Define slot mapping contract.
  - Detail: Add checker fixtures.
  - Detail: Promote only selected window row.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.inventory_extra_window_types.artifacts]
- [ ] [depends:artifacts] Promote only the `extra inventory window types` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.inventory_extra_window_types.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.inventory_extra_window_types.validation]
