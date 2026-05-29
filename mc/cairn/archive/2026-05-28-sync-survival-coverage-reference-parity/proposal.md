# Proposal: Sync survival coverage matrix with reference parity

## Why

The survival coverage matrix still says the break/place/pickup row has Valence-only evidence and missing reference evidence, but paired Paper and Valence reference-parity evidence has since been promoted and archived. This stale row can mislead future survival work and undercuts the current acceptance matrix.

## What Changes

- Update the survival coverage matrix so break/place/pickup is marked as paired reference parity covered.
- Tighten the survival coverage checker so the promoted row must cite both Paper/reference and Valence receipts plus the parity evidence doc.
- Keep every broader survival row as a non-claim.
- Record validation evidence and archive the correction.

## Impact

- **Files**: `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md`, `tools/check_survival_coverage_matrix.py`, evidence manifests/logs, accepted/archived Cairn specs.
- **Testing**: survival coverage checker self-test/check, survival parity checker, acceptance matrix, current bundle, evidence manifests, Cairn validation.
