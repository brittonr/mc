# Proposal: Extract Valence compatibility fixture cores

## Why

`servers/valence/examples/ctf.rs` and `servers/valence/examples/survival_compat.rs` have grown from examples into compatibility fixtures with game rules, rail-specific probes, milestone formatting, and mutable policy state. Keeping this logic inside monolithic examples makes fixture behavior hard to test without a running Bevy app and encourages global-state shortcuts.

## What Changes

- Inventory CTF and survival fixture rule logic, Bevy shell code, milestone formatting, global state, and non-goals.
- Extract deterministic rule/fixture cores for CTF scoring/flags, inventory probes, combat probes, survival fixture decisions, and milestone formatting.
- Replace global policy state with explicit Bevy resources or fixture state where practical.
- Keep examples as thin Bevy adapters that wire events/resources to pure fixture cores.
- Preserve existing fixture milestones and mc-compat evidence boundaries.

## Impact

- **Files**: `servers/valence/examples/ctf.rs`, `servers/valence/examples/survival_compat.rs`, new fixture modules/crates as appropriate, tests, docs/evidence after implementation.
- **Testing**: Valence focused tests, example compile checks, selected mc-compat dry-runs/live rails when required, positive/negative fixture-core tests, and Cairn gates.
