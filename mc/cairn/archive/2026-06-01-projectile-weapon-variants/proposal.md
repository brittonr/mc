# Proposal: Projectile weapon variants rail

## Why

Projectile evidence currently covers a narrow use/loadout and pinned damage attribution row. Other projectile weapon variants remain non-claims.

## What Changes

- Add `projectile-weapon-variants` as a row-scoped Cairn for a bounded matrix of configured projectile weapons with use, spawn, hit/miss, damage or no-damage outcome, and per-weapon non-claims.
- Define normalized metrics: weapon id, ammunition/item state, use action, projectile spawn, target identity, hit/miss outcome, damage delta when applicable, and server correlation.
- Require evidence standard: matrix checker with one live receipt per promoted weapon row and optional reference parity only when paired evidence exists.
- Reject bad evidence and overclaims: missing weapon id, missing projectile spawn, wrong target/outcome, missing damage/no-damage metric, all-weapons overclaim, or exact vanilla physics overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: all projectile weapons, projectile travel physics, exact vanilla projectile parity, enchantments/status effects, combat balancing, and production readiness.
