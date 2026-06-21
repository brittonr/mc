# Proposal: Survival furnace smelting breadth parity

## Why

The current furnace row proves one RawIron plus Coal flow with reconnect persistence. Survival parity still needs bounded evidence for additional smelting/fuel behavior and timing before broader furnace claims are possible.

## What Changes

- Add a scoped `survival-furnace-smelting-breadth-parity` row for a finite smelting matrix.
- Require paired Paper/reference and Valence receipts for configured recipe/fuel variants, burn/cook progress observations, output collection, and one invalid-fuel or insufficient-input rejection.
- Add deterministic checker coverage that rejects Valence-only evidence, missing burn/cook metrics, mismatched output, stale revisions, and all-furnace overclaims.
- Promote only the bounded furnace-breadth row after comparator evidence passes.

## Impact

- **Files**: scenario manifest, runner/client furnace rail, Paper survival fixture, Valence `survival_compat`, row checker, evidence docs, receipts, and manifests.
- **Testing**: positive and negative checker fixtures, paired live receipts, dry-run shape checks, evidence manifest checks, task-evidence gate, Cairn gates, and Cairn validation.

## Non-Claims

No all smelting recipes, all fuels, hopper automation, furnace minecarts, long-running timing parity, chunk-unload behavior, full survival compatibility, broad vanilla parity, public-server safety, or production readiness is claimed.
