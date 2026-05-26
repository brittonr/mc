# Proposal: Residual combat non-claim catalog

## Summary

Drain ROI-ranked maintenance slice `roi-03-residual-combat-catalog` for the Minecraft compatibility evidence workspace.

## Motivation

The protocol-763 evidence set is clean and landed; this slice keeps the maintained evidence, operator surfaces, and lifecycle state accurate before further feature work.

## Scope

- Update only the narrow documentation/checker/harness surfaces needed for this ROI slice.
- Preserve scoped non-claims and avoid broad compatibility claims.
- Verify with deterministic local checks.

## Non-goals

- No public-server load testing.
- No broad Minecraft compatibility claim.
- No unrelated child-repo changes unless the slice explicitly requires a runner source update.
