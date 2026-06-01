# Tasks

- [x] [serial] Define the bounded `inventory drag transactions` evidence contract and normalized metric names. r[mc_compatibility.inventory_drag_transactions.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for `inventory drag transactions`. r[mc_compatibility.inventory_drag_transactions.checker]
- [ ] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.inventory_drag_transactions.evidence_standard]
- [ ] [depends:evidence_standard] Add `inventory-drag-transactions` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.inventory_drag_transactions.rail]
  - Detail: Define drag phase contract.
  - Detail: Add phase-order checker fixtures.
  - Detail: Add runner/client drag probe.
  - Detail: Promote only configured drag row.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.inventory_drag_transactions.artifacts]
- [ ] [depends:artifacts] Promote only the `inventory drag transactions` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.inventory_drag_transactions.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.inventory_drag_transactions.validation]
