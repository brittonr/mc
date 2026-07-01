# Proposal: Add gameplay plugin template helper

## Why

Valence examples repeat the same plugin boilerplate: phase enums, contract resources, shared metadata registration, schedule setup, install tests, and disabled-plugin negative tests. Repetition makes new plugins easy to half-register and causes contract drift between examples.

## What Changes

- Add a reusable gameplay plugin template/helper for selected Valence examples.
- Capture common contract metadata, phase wiring, schedule setup, and test helper patterns in one place.
- Provide a documented path for new gameplay/example plugins to register contracts and prove disabled behavior.
- Migrate selected examples incrementally without changing their visible behavior.

## Impact

- **Files**: Valence example helper modules, selected examples, tests, documentation/inventory, and evidence logs.
- **Testing**: helper unit tests, positive plugin-install tests, negative missing-contract/disabled-plugin tests, selected example checks, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests.
- **Non-claims**: this does not add a macro-only framework, default gameplay, runtime plugin loading, or production plugin marketplace; it reduces boilerplate for compiled Bevy plugins.
