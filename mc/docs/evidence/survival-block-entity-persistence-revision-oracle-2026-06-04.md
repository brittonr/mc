# Survival sign block-entity persistence revision oracle — 2026-06-04

## Question

Which child revisions back the promoted `survival-block-entity-persistence-parity` row?

## Inspected evidence

- Valence fixture revision: `f54e6d01b23a08a2977c7bf2d93301d589c65c4d` (`persist bounded sign fixture state`).
- Stevenarella client revision: `79d766c2439120ba2ec0217dd88ee9f708db9844` (`observe persisted sign block entities`).
- Paper fixture source BLAKE3: `864fedc1f1f645058b9ca061829d7c42e3da8d088e8cc394ab0c8abf6b2f5150`.
- Paper fixture jar BLAKE3: `c521ee9bc862cfb9be8fb6ffcbf3862cba6515869254db080e2a9ea46837e080`.
- Paired live run log: `docs/evidence/survival-block-entity-persistence-paired-live-2026-06-04.run.log` records `valence_live_exit_status=0`, `paper_live_exit_status=0`, and `exit_status=0`.

## Decision

Promote only the bounded sign payload row. The child revision metadata is clean for the Valence fixture and Stevenarella client, and the Paper fixture is pinned by source/jar BLAKE3 digests.

## Owner

MC compatibility evidence owner.

## Next action

Keep all-block-entity parity, arbitrary NBT parity, sign editing UI parity, broad survival compatibility, public-server safety, and production readiness as non-claims unless future paired rows add explicit evidence.
