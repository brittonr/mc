# Proposal: Survival sign editing live parity

## Why

The current survival sign row proves persisted sign text, and a targeted packet fixture covers sign editor open/update shape. Survival parity still lacks paired live Paper/reference and Valence evidence for the actual sign editing interaction.

## What Changes

- Add a scoped `survival-sign-editing-live-parity` row for one configured sign editing session.
- Require paired Paper/reference and Valence receipts for sign editor open, submitted text payload, server acceptance, client post-update observation, restart or reconnect observation when configured, and forbidden mismatch scans.
- Add deterministic checker coverage that rejects Valence-only evidence, missing open/update metrics, mismatched text/position/side, stale revisions, and all-sign UI overclaims.
- Promote only the bounded sign-editing row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, Stevenarella sign-edit driver path, runner/client rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifests, task gate, Cairn gates, and Cairn validation.

## Non-Claims

No all sign UI behavior, all sign variants, all text formatting, arbitrary NBT parity, all block entities, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
