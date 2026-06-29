# Proposal: Split Valence command example

## Why

`servers/valence/examples/command.rs` combines example app setup, command graph construction, handlers, fixture data, and tests. As a command API teaching surface, it should separate reusable example command definitions from Bevy app shell and test fixtures.

## What Changes

- Split the command example into modules for app setup, command definitions, handler logic, fixture/test helpers, and example documentation comments.
- Extract pure command definition/handler decisions where practical.
- Keep Bevy app wiring, client/entity mutation, packet/event side effects, and logging in shells.
- Preserve example behavior, documented command API usage, handler outcomes, tests, and non-claims.

## Impact

- **Files**: `servers/valence/examples/command.rs`, optional command example modules, focused tests, and Cairn artifacts.
- **Testing**: baseline command example tests, positive and negative handler/definition tests, affected example smoke checks, Cairn gates, and Cairn validation.
- **Non-claims**: example organization only; no new command framework behavior or compatibility claim is promoted.
