# Design: Survival coverage matrix

## Matrix rows

Initial rows:

- break/place/pickup;
- crafting;
- chest interaction and persistence;
- furnace smelting and persistence;
- hunger/food health loop;
- mob spawn/drop interaction;
- redstone power/update;
- biome and dimension transition/persistence.

Each row records status, required receipts, required child revisions, explicit non-claims, owner, and next action.

## Gate

The checker reads the matrix and fails if any documentation claims full survival compatibility while any required row is missing live Valence evidence, paired reference evidence where parity is claimed, or BLAKE3-backed logs.

## Drain strategy

Future implementation should drain one row per change unless two rows share the same fixture and oracle. The matrix is the coordinator, not the evidence itself.
