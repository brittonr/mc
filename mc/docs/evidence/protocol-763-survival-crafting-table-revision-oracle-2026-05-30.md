# Survival crafting-table revision oracle checkpoint

## Question

If a future `survival-crafting-table` Paper or Valence receipt cannot machine-record child repository revision metadata, what reviewable checkpoint may the checker accept instead?

## Inspected evidence

- `tools/check_survival_crafting_table.rs` requires committed child revision metrics by default.
- The only accepted fallback value is `docs/evidence/protocol-763-survival-crafting-table-revision-oracle-2026-05-30.md`.
- No promoted live crafting-table receipt currently relies on this fallback.

## Decision

Use this file only as a narrow oracle checkpoint for receipts whose child revision metadata is unavailable but whose reviewer has recorded the missing revision evidence here. Receipts with arbitrary oracle paths, dirty revisions, dry-run revisions, missing revisions, or mismatched Valence requested/resolved revisions remain rejected.

## Owner

Current compatibility evidence maintainer.

## Next action

Prefer machine-recorded `client.git_rev`, `client.git_status`, `client.git_dirty`, `valence.git_rev_requested`, `valence.git_rev_resolved`, `valence.git_status`, and `valence.git_dirty` in every live crafting-table receipt. If that is impossible, append the inspected child-revision evidence and reviewer decision to this checkpoint before promoting the receipt.
