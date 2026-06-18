## Why

`entity-status-effect-packets` is a targeted packet row with strong gameplay value, but it should not inherit coverage from combat or survival rows. A bounded live rail can prove one effect apply/remove observation without claiming effect mechanics or modifier parity.

## What Changes

- Add an isolated status-effect live rail for one configured entity/effect/amplifier/duration scope.
- Record packet row identity, client observation, Valence server correlation, child revisions when available, and explicit non-claims.
- Validate normalized evidence with positive and negative checker fixtures before any matrix or packet-inventory promotion.
- Keep all effects, stacking, particles/UI, gameplay modifiers, combat balancing, survival parity, public-server safety, and production readiness as non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, `tools/check_targeted_packet_promotions.rs`, `docs/evidence/**`, acceptance matrix/current bundle/packet inventory if live evidence passes.
- **Testing**: Baseline targeted packet checks, runner unit/dry-run checks, status-effect evidence checker positive/negative fixtures, evidence-manifest/task-evidence checks, Cairn gates and validation.
