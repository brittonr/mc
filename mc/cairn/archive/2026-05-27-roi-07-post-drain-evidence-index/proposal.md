# Proposal: Post-drain evidence index

## Summary

Index the already-drained ROI 01–03 evidence receipts in the maintained protocol-763 acceptance matrix, current evidence bundle, residual combat catalog, and local checkers.

## Motivation

ROI 01 armor/equipment mitigation, ROI 02 equipment update observation, and ROI 03 projectile rail receipts are tracked and manifest-checked, but the higher-level evidence indexes still describe those seams as open or residual. That makes future work likely to repeat drained seams or over-read stale non-claims.

## Scope

- Add bounded ROI 01–03 rows to the acceptance matrix.
- Mirror those rows in the current evidence bundle.
- Update the residual combat catalog so covered rails are discoverable while remaining non-claims stay explicit.
- Update checker expectations for the new row count and required seam names.
- Add a small index note tying the matrix rows back to tracked receipts and BLAKE3 manifests.

## Non-goals

- No new live compatibility run.
- No expanded claim of full projectile physics, all equipment slots, all armor loadouts, enchantments, or full combat correctness.
- No changes to Valence, Stevenarella, or runner behavior.
