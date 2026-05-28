# Tasks

Review oracle: `docs/evidence/protocol-763-survival-coverage-matrix-oracle-2026-05-28.md` records the inspected matrix/checker evidence, decision, owner, and next action for review paths that omit implementation files.

- [x] [serial] Add survival coverage matrix document with current break/place/pickup row and uncovered survival rows. r[mc_compatibility.survival_coverage_matrix.rows]
  - Evidence: Added `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` with 9 survival system rows: break/place/pickup plus crafting, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence.
- [x] [serial] Add checker positive and negative tests for missing rows and forbidden full-survival claims. r[mc_compatibility.survival_coverage_matrix.gate]
  - Evidence: Added `tools/check_survival_coverage_matrix.py`; `--self-test` covers a passing matrix, missing row, incorrectly promoted row, and forbidden full-survival claim.
- [x] [depends:rows] Add row templates for crafting, chest persistence, furnace persistence, hunger/food, mob drops, redstone, and biome/dimension seams. r[mc_compatibility.survival_coverage_matrix.row_requirements]
  - Evidence: Matrix rows define promotion requirements and next actions for each uncovered survival system.
- [x] [depends:gate] Wire the checker into maintained evidence checks. r[mc_compatibility.survival_coverage_matrix.gate]
  - Evidence: `docs/evidence/protocol-763-current-evidence-bundle.md` lists `python3 tools/check_survival_coverage_matrix.py`; the checker passes after implementation.
- [x] [depends:rows] Update current bundle/matrix docs to point to the survival coverage matrix for full-survival non-claims. r[mc_compatibility.survival_coverage_matrix.nonclaims]
  - Evidence: Current bundle and acceptance matrix cite the survival coverage matrix as the reason full survival compatibility/vanilla parity remain non-claims.
