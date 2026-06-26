# Proposal: Organize remaining Valence examples as Bevy plugins

## Why

The CTF, survival compatibility, and terrain examples already expose opt-in Bevy plugins with named phase sets. Many other examples still wire systems directly from `main`, including parkour, game of life, combat, command, advancement, building, death, and world border examples. That makes schedule boundaries harder to inspect and gives downstream users fewer reusable examples for composing gameplay as plugins.

## What Changes

- Inventory selected remaining examples, including systems, schedules, resources, events, env/config inputs, milestones, pure helpers, and non-goals.
- Define example-local plugin boundaries and named `SystemSet` phase contracts for input, rule evaluation, world mutation, presentation, and cleanup where applicable.
- Move selected direct `App::new().add_systems(...)` wiring into opt-in plugins while keeping deterministic gameplay decisions in pure helpers.
- Preserve example command names, CLI/env contracts, visible behavior, and README-called-out example behavior unless another Cairn changes them.
- Add positive plugin/schedule smoke tests and negative disabled-plugin or ordering-regression tests.

## Impact

- **Files**: selected files under `servers/valence/examples/`, focused example tests, schedule evidence under `docs/evidence/`.
- **Testing**: focused `cargo test --example ...` checks, selected smoke runs when behavior changes, Valence schedule hygiene, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not promote examples into default Valence gameplay, add vanilla parity, or claim production readiness.
