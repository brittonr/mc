# Proposal: Gate optional fixtures with Bevy run conditions

## Why

Valence compatibility examples contain many optional fixture and probe systems controlled by environment toggles or optional resources. Several systems run every tick only to immediately return when a fixture is disabled. Bevy `run_if` conditions can make disabled behavior visible in schedule evidence and avoid unnecessary system bodies, but event-reader systems require careful cursor semantics so disabled periods do not replay stale inputs when re-enabled.

## What Changes

- Inventory selected optional fixture/probe systems, runtime enabled checks, event readers, resources, disabled behavior, and re-enable expectations.
- Classify disabled behavior as skip, drain, transform, reject, or explicit in-system guard before moving any guard into `run_if`.
- Add Bevy run conditions or set-level conditions only for systems whose disabled behavior is a pure no-op.
- Keep explicit in-system drains for event-reader systems that must advance cursors while disabled.
- Add positive enabled tests and negative disabled, stale-event, and runtime-toggle tests.

## Impact

- **Files**: selected systems in `servers/valence/examples/survival_compat.rs`, selected CTF probe systems if scoped, fixture pure-core helpers if needed, schedule/evidence docs.
- **Testing**: focused example tests, selected mc-compat dry-runs if fixture behavior changes, stale-event tests, Valence schedule hygiene, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not change fixture milestone contracts, add production gameplay policy, or make every optional system eligible for `run_if`.
