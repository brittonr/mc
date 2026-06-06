# Proposal: Promote entity status-effect packet evidence

## Why

Status-effect packets remain uncovered even though combat and survival rows already use health, damage, and equipment updates. A bounded apply/remove effect row would add high-value protocol coverage while keeping all potion/effect gameplay semantics out of scope.

## What Changes

- Add one bounded status-effect packet row for a configured actor, target, effect id/name, amplifier, duration, and removal.
- Require client observation of apply/remove events and Valence server correlation.
- Promote only the configured `EntityStatusEffectS2CPacket` and optional `RemoveEntityStatusEffectS2CPacket` row, keeping all effects, stacking, particles, gameplay modifiers, full combat/survival parity, and production readiness as non-claims.

## Impact

- **Files**: Valence fixture instrumentation, Stevenarella observation probe, runner metadata, packet inventory/current bundle docs, checker, evidence artifacts, and Cairn specs/tasks.
- **Testing**: positive/negative checker fixtures, focused scenario tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
