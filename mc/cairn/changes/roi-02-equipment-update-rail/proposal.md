# Proposal: Equipment update observation rail

## Summary

Create ROI-ranked compatibility change `roi-02-equipment-update-rail` for the Minecraft protocol-763 Stevenarella ⇄ Valence CTF evidence workspace.

## Motivation

The current evidence set is clean and archived at parent `1c5880a` with 11 maintained seams. The residual catalog still marks this area as outside the proven compatibility boundary. This change gives the next drain a bounded, reviewable lifecycle package before mutating runner, Valence, Stevenarella, or evidence surfaces.

## Scope

- Keep work scoped to the maintained local Valence CTF / Stevenarella protocol-763 harness.
- Add or update runner, flake, child-repo instrumentation, and tracked evidence only as required by this slice.
- Preserve BLAKE3-backed receipts, deterministic dry-run checks, and explicit non-claims.

## Non-goals

- No public-server or unauthorized load testing.
- No broad Minecraft compatibility claim.
- No full CTF/combat correctness claim beyond the bounded receipt.
- No unrelated Leafish or Hyperion implementation work.
