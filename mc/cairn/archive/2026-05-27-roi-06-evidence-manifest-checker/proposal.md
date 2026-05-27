# Proposal: Evidence manifest checker

## Summary

Add a local gate that verifies tracked `docs/evidence/*.b3` manifests and rejects stale receipt milestone names before Cairn evidence is archived or reviewed.

## Motivation

Recent Cairn drain review failures came from evidence that was either untracked, stale, or inconsistent with updated receipt semantics. A small maintained checker can catch these review-blocking mistakes before commit.

## Scope

- Add a repo-local evidence manifest checker for tracked `docs/evidence/*.b3` files.
- Verify referenced files exist and BLAKE3 digests match manifest entries.
- Reject stale receipt milestone names known to have caused review drift, starting with `equipment_packet_observed`.
- Wire the checker into current maintained checks and evidence bundle documentation.

## Non-goals

- No new live Minecraft compatibility rail.
- No broad rewrite of evidence docs.
- No claim that BLAKE3 manifests prove semantic correctness beyond file integrity and stale-marker guards.
