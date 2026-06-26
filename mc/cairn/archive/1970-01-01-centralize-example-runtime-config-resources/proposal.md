# Proposal: Centralize example runtime config as Bevy resources

## Why

Compatibility and gameplay examples read environment variables and filesystem paths from many helper functions. That scatters runtime policy across systems, makes disabled behavior harder to schedule with Bevy, and complicates tests. Loading runtime configuration once into typed Bevy resources would make config ownership explicit while preserving existing env var contracts.

## What Changes

- Inventory selected example env/CLI/config reads, filesystem path inputs, reload triggers, default values, and milestone effects.
- Define typed runtime config resources and pure parsers that convert explicit inputs into validated config values.
- Replace repeated `std::env` reads in selected systems with resource access or explicit reload-event handling.
- Preserve existing env var names, CLI inputs, default behavior, and compatibility milestone text unless another Cairn changes them.
- Add positive config parsing/resource installation tests and negative missing, malformed, conflicting, reload-stale, and disabled-plugin tests.

## Impact

- **Files**: selected systems in `servers/valence/examples/ctf.rs`, `servers/valence/examples/survival_compat.rs`, possibly `terrain.rs` seed handling, fixture pure-core helpers, and evidence docs.
- **Testing**: pure config parser tests, resource installation tests, selected example/compatibility dry-runs when behavior changes, schedule hygiene if run conditions or plugin wiring change, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not change env var contracts, introduce a new external config format, or add production configuration management.
