# Tasks

- [x] [serial] Define the bounded `equipment slot/item matrix` evidence contract and normalized metric names. r[mc_compatibility.equipment_slot_item_matrix_expansion.contract]
  - Evidence: `docs/evidence/protocol-763-equipment-slot-item-expansion-contract-2026-05-29.md` defines bounded candidate rows, normalized metrics, promotion requirements, reject cases, and non-claims; verification output is in `docs/evidence/protocol-763-equipment-slot-item-expansion-contract-2026-05-29.run.log`; BLAKE3 manifest `docs/evidence/protocol-763-equipment-slot-item-expansion-contract-2026-05-29.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `equipment slot/item matrix`. r[mc_compatibility.equipment_slot_item_matrix_expansion.checker]
- [ ] [depends:checker] Add the `equipment-slot-item-matrix-expansion` runner, fixture, or parser rail without broadening existing scenarios. r[mc_compatibility.equipment_slot_item_matrix_expansion.rail]
  - Detail: Define equipment matrix row schema.
  - Detail: Add checker positive/negative fixtures.
  - Detail: Add expanded observer rail.
  - Detail: Promote only listed slot/item rows.
- [ ] [depends:rail] Produce reviewable row evidence under `docs/evidence/`, including receipts/logs/check output and BLAKE3 manifests. r[mc_compatibility.equipment_slot_item_matrix_expansion.evidence]
- [ ] [depends:evidence] Promote only the `equipment slot/item matrix` row and keep adjacent non-claims explicit in matrix/current-bundle docs. r[mc_compatibility.equipment_slot_item_matrix_expansion.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checker, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.equipment_slot_item_matrix_expansion.validation]
