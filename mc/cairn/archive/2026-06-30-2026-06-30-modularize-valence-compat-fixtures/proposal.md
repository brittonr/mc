# Proposal: Modularize Valence compatibility fixture examples

## Why

`servers/valence/examples/ctf.rs` and `servers/valence/examples/survival_compat.rs` are large runnable examples that also own compatibility fixture contracts, runtime configuration, Bevy plugin wiring, probe state, typed milestone logging, gameplay systems, and safety/non-claim boundaries. The examples should be thin shells over focused fixture modules so gameplay and probe changes can be reviewed without scanning thousands of unrelated lines.

## What Changes

- Inventory current CTF and survival fixture responsibilities, plugin/schedule ownership, scenario env contracts, and baseline checks.
- Move fixture-specific pure logic and Bevy systems into focused modules under `examples/ctf/*`, `examples/survival_compat/*`, `examples/fixture_core/*`, or a dedicated fixture helper crate if justified.
- Keep runnable example entrypoints thin: configure the app, install explicit opt-in plugins, and delegate to fixture modules.
- Preserve schedule/plugin contracts, scenario env vars, typed milestone vocabulary, fixture state, gameplay behavior, non-claims, and runner dry-run/live receipt shapes.
- Add positive and negative tests for extracted pure fixture logic, plugin/schedule contracts, invalid env/config cases, and scenario-specific probe behavior.

## Impact

- **Files**: `servers/valence/examples/ctf.rs`, `servers/valence/examples/survival_compat.rs`, `servers/valence/examples/ctf/*`, `servers/valence/examples/fixture_core/*`, optional fixture helper modules/crate, schedule hygiene checks, and Cairn artifacts.
- **Testing**: baseline and post-change Valence example/fixture tests, schedule hygiene, affected mc-compat dry-runs/live rails when claimed by tasks, Cairn gates, and Cairn validation.
- **Non-claims**: fixture maintainability and schedule clarity only; this does not claim broader Valence gameplay correctness, vanilla parity, public-server safety, or production readiness.
