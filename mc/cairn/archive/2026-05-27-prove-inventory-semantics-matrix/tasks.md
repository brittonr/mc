# Tasks

- [x] [serial] Define the inventory semantics matrix and slot mapping table. r[mc_compatibility.prove_inventory_semantics_matrix.inventory_matrix]
- [x] [serial] Add positive valid-interaction scenarios. r[mc_compatibility.prove_inventory_semantics_matrix.positive_inventory_scenarios]
- [x] [serial] Add negative stale/invalid interaction scenarios. r[mc_compatibility.prove_inventory_semantics_matrix.negative_inventory_scenarios]
- [x] [serial] Promote only rows with correlated client/server receipts. r[mc_compatibility.prove_inventory_semantics_matrix.inventory_promotion_gate]

## Progress

- Inventory matrix and slot mapping are documented in `docs/evidence/protocol-763-inventory-semantics-matrix-2026-05-27.md`.
- Positive validation covers five protocol-763 live receipt-backed rows: drop, pickup, player-inventory click, open-container click, and block placement/use-item-on-block.
- Negative fixtures in `tools/check_inventory_semantics_matrix.py --self-test` reject missing client/server milestones, protocol mismatch, stale state acceptance, server state corruption, and missing matrix/non-claim model text.
- `tools/check_inventory_semantics_matrix.py` promotes only rows with correlated client/server receipts and keeps full inventory semantics as a non-claim.
