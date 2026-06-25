# Proposal: Gate optional systems with Bevy run conditions

## Why

Optional Valence hooks should be cheap and explicit when disabled. Some plugin shells currently install systems that check a config resource every run. Bevy run conditions can make disabled-mode behavior more visible, reduce avoidable work, and keep optional plugins from consuming events unless that is part of their disabled contract.

## What Changes

- Inventory optional plugins and systems that perform runtime `enabled` checks, especially observability and compatibility helper systems.
- Define disabled-mode contracts for each event reader: skip, drain, or transform.
- Replace suitable per-system config guards with Bevy `run_if` conditions or explicit enabled/disabled system sets.
- Keep fail-closed draining systems where skipped readers would accumulate stale events.
- Add positive enabled tests and negative disabled, stale-event, and config-toggle tests.

## Impact

- **Files**: `servers/valence/crates/valence_server/src/observability.rs`, optional plugin shells, tests, docs/evidence after implementation.
- **Testing**: enabled/disabled plugin tests, event-reader stale backlog tests, config toggle tests, focused Valence checks, Cairn gates, and Cairn validation.
- **Non-claims**: this does not require every optional branch to use `run_if`; explicit in-system checks remain valid when they are part of an event-drain or fail-closed contract.
