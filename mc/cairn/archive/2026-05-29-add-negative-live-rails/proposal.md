# Proposal: Add negative live rails

## Why

Several important invalid behaviors are currently explicit non-claims or checker-only fixtures: stale inventory state id rejection, invalid slot/window clicks, malformed custom payloads, reconnect races, and wrong team/score paths. Deterministic unit checks are useful, but live negative rails are needed before the harness can claim runtime rejection/restoration behavior.

## What Changes

- Add a bounded owned-local negative injection envelope for invalid client actions and malformed packet fixtures.
- Produce live receipts that prove the server rejects, restores, or contains each invalid action without broadening production/public-network claims.
- Start with high-ROI negative rails: stale inventory state id, invalid slot/window click, malformed custom payload, reconnect flag race, and wrong team/score path.
- Require explicit non-claims for any invalid behavior not covered by a live rail.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe code, Valence/Paper fixtures where reference parity applies, evidence checkers, README/current bundle.
- **Testing**: runner unit tests, dry-run negative receipt checks, live owned-local receipts, positive and negative checker fixtures.
- **Safety**: rails run only against owned local loopback fixtures with bounded client count and timeout.
