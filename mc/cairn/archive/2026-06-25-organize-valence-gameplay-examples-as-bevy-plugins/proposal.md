# Proposal: Organize Valence gameplay examples as Bevy plugins

## Why

Valence examples such as `ctf.rs`, `survival_compat.rs`, and `terrain.rs` demonstrate real gameplay and compatibility fixture behavior, but their `App` setup still wires large system tuples directly in `main`. That makes schedule ordering harder to inspect, encourages monolithic examples, and hides the boundary between gameplay policy, compatibility fixture adapters, and Bevy ECS orchestration.

## What Changes

- Inventory current example `App` setup, Bevy schedules, system ordering, resources, events, env toggles, and compatibility milestones.
- Extract selected example wiring into opt-in Bevy plugins with named `SystemSet`s for input, rule evaluation, world mutation, presentation, and cleanup phases.
- Keep deterministic fixture/gameplay decisions in pure modules; plugins remain thin ECS shells.
- Preserve example commands, env var contracts, milestone text, and mc-compat evidence boundaries.
- Add positive schedule/plugin smoke tests and negative disabled-plugin or ordering regression tests.

## Impact

- **Files**: `servers/valence/examples/ctf.rs`, `servers/valence/examples/survival_compat.rs`, `servers/valence/examples/terrain.rs`, fixture modules, tests, docs/evidence after implementation.
- **Testing**: focused example checks, plugin-disabled smoke tests, schedule/order assertions, selected mc-compat dry-runs where fixture behavior changes, Cairn gates, and Cairn validation.
- **Non-claims**: this does not make CTF, survival, or terrain behavior production-ready or vanilla-parity gameplay by default.
