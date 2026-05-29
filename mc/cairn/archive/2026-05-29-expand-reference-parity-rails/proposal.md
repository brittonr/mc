# Proposal: Expand reference parity rails

## Why

Survival break/place/pickup now has paired Paper and Valence evidence, but most fidelity-sensitive claims still lack a reference backend. Docs correctly keep full survival compatibility, vanilla parity, crafting, furnace, hunger/food, mob drops, redstone, biome/dimension, world persistence, and vanilla combat parity as non-claims. To improve trust, the harness should standardize paired reference rails for any claim that implies vanilla-like behavior.

## What Changes

- Define a reference-parity policy for when a claim requires paired Paper/reference and Valence receipts.
- Extend the survival parity pattern beyond break/place/pickup, coordinating with the active chest-persistence change.
- Add normalized comparators for survival rows and vanilla combat rows.
- Require matrix/current-bundle updates to distinguish reference parity, Valence-only containment, and non-claims.

## Impact

- **Files**: Paper fixture plugin, Valence fixtures, runner scenarios, reference parity checkers, survival/combat matrices, README/current bundle.
- **Testing**: comparator positive/negative fixtures, paired dry-run/live receipts, Valence-only rejection fixtures, evidence manifest checks.
- **Non-claims**: each row remains narrow; full survival compatibility and full vanilla parity stay false until all required rows pass.
