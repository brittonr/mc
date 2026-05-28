# Proposal: Build survival coverage matrix

## Summary

Create a survival coverage matrix and gates that track which survival systems have live Valence/reference evidence and which remain non-claims.

## Motivation

The current survival rail covers only break/place/pickup. Crafting, furnace, chests, hunger, mobs, redstone, biome/dimension behavior, and persistence are currently uncovered. A matrix prevents accidental promotion from one rail to full survival compatibility.

## Scope

- Add a survival coverage ledger with rows for the uncovered systems.
- Add a checker that blocks `full_survival_compatibility` unless every required row has evidence.
- Define per-row evidence requirements so future rails can be drained one at a time.
- Seed next-ROI tasks for crafting, chest/persistence, furnace, hunger/food, mob drops, redstone, and biome/dimension seams.

## Non-goals

- No immediate claim that all rows are implemented.
- No vanilla parity unless paired reference receipts exist.
- No broad protocol-763 coverage claim.
