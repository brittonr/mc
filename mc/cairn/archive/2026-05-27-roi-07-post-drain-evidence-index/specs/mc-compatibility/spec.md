# Delta: Post-drain evidence index

## Requirements

### Requirement: Matrix index

r[mc_compatibility.roi_07_post_drain_evidence_index.matrix_index] The protocol-763 acceptance matrix MUST index the tracked ROI 01 armor/equipment mitigation, ROI 02 equipment update observation, and ROI 03 projectile rail receipts with BLAKE3 hashes, scoped claims, and explicit non-claims.

#### Scenario: Drained ROI receipts are visible in the matrix

r[mc_compatibility.roi_07_post_drain_evidence_index.matrix_index.scenario]
- GIVEN reviewable ROI 01–03 receipt copies exist under `docs/evidence`
- WHEN an operator reads `docs/evidence/protocol-763-acceptance-matrix.md`
- THEN the matrix lists armor/equipment mitigation, equipment update observation, and projectile use/loadout evidence rows
- AND each row includes a maintained command, receipt path, BLAKE3 digest, scoped claim, and explicit non-claim

### Requirement: Bundle alignment

r[mc_compatibility.roi_07_post_drain_evidence_index.bundle_alignment] The current evidence bundle MUST mirror the acceptance matrix seam names and BLAKE3 hashes for the indexed ROI 01–03 rows.

#### Scenario: Bundle mirrors new rows

r[mc_compatibility.roi_07_post_drain_evidence_index.bundle_alignment.scenario]
- GIVEN the acceptance matrix includes ROI 01–03 rows
- WHEN `tools/check_current_evidence_bundle.py` runs
- THEN the bundle row count matches the matrix row count
- AND each new seam has the same BLAKE3 digest in both files

### Requirement: Residual catalog alignment

r[mc_compatibility.roi_07_post_drain_evidence_index.residual_catalog_alignment] The residual combat catalog MUST treat ROI 01–03 as covered bounded rails while preserving exact remaining combat non-claims.

#### Scenario: Residual guidance is not stale

r[mc_compatibility.roi_07_post_drain_evidence_index.residual_catalog_alignment.scenario]
- GIVEN ROI 01–03 receipts are indexed
- WHEN an operator reads `docs/evidence/protocol-763-residual-combat-catalog.md`
- THEN those seams appear as covered combat rails
- AND the remaining non-claims focus on unproven breadth such as all equipment slots, armor loadouts, enchantments, projectile travel/collision/damage, and full combat correctness

### Requirement: Checker coverage

r[mc_compatibility.roi_07_post_drain_evidence_index.checker_coverage] Local evidence checkers MUST fail if the newly indexed rows disappear or bundle row counts drift.

#### Scenario: Checkers guard the indexed rows

r[mc_compatibility.roi_07_post_drain_evidence_index.checker_coverage.scenario]
- GIVEN ROI 01–03 rows are part of the maintained evidence set
- WHEN `tools/check_acceptance_matrix.py` and `tools/check_current_evidence_bundle.py` run
- THEN they require the new seam names and expected matrix/bundle row count
