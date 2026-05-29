# Tasks

- [x] [serial] Update the survival coverage matrix break/place/pickup row to cite paired Paper and Valence reference evidence. r[mc_compatibility.survival_coverage_matrix.reference_parity_synced]
  - Evidence: `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` marks `break/place/pickup` as `reference_parity_covered` and cites the Paper reference receipt, Valence paired receipt, and parity doc.
- [x] [serial] Tighten the survival coverage checker with positive and negative fixtures for the paired reference row and stale Valence-only/reference-missing states. r[mc_compatibility.survival_coverage_matrix.reference_parity_gate]
  - Evidence: `tools/check_survival_coverage_matrix.py --self-test` now covers the promoted paired row, stale reference-missing status, missing Paper receipt, missing required survival row, unimplemented row promotion, and full-survival overclaim rejection.
- [x] [depends:reference_parity_gate] Refresh evidence docs/manifests and keep broader survival rows as non-claims. r[mc_compatibility.survival_coverage_matrix.reference_parity_nonclaims]
  - Evidence: `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` keeps crafting, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence as `missing` rows with explicit non-claims.
- [x] [depends:nonclaims] Run verification and archive the change. r[mc_compatibility.survival_coverage_matrix.reference_parity_validation]
  - Evidence: `docs/evidence/protocol-763-survival-coverage-reference-parity-sync-2026-05-28.run.log` records checker, matrix, bundle, manifest, Cairn gate, and Cairn validation output.
