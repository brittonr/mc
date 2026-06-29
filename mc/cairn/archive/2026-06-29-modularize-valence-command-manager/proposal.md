# Proposal: Modularize Valence command manager

## Why

`servers/valence/crates/valence_command/src/manager.rs` combines plugin setup, packet event adapters, command tree updates, incoming command parsing, argument parsing, event emission, and tests. Command routing and parse decisions should be pure cores, while Bevy systems and packet/event shells stay thin.

## What Changes

- Split command manager code into modules for plugin wiring, packet adapters, command tree sync, parse core, execution event planning, and Bevy systems.
- Extract pure decisions for packet-to-command events, command tree update needs, command parse outcomes, argument parse plans, and processed-event plans.
- Keep Bevy queries/resources/events, packet sends, and schedule wiring in shells.
- Preserve public command APIs, event shapes, command tree behavior, parsing behavior, schedule behavior, and non-claims.

## Impact

- **Files**: `servers/valence/crates/valence_command/src/manager.rs`, command manager modules, focused tests, schedule checks where needed, and Cairn artifacts.
- **Testing**: baseline Valence command tests, positive and negative command-manager tests, affected examples/checks, Cairn gates, and Cairn validation.
- **Non-claims**: command manager architecture only; no new command semantics or compatibility claim is promoted.
