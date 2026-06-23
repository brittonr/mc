# Proposal: Add an optional GUI helper plugin

## Why

Hyperion has early GUI helper code for inventory-backed clickable screens. Valence already has a richer inventory crate, so the useful integration is a Valence-native helper that makes common menu screens easier while preserving the existing inventory model and packet semantics.

## What Changes

- Review Hyperion GUI helper concepts and Valence inventory/window behavior.
- Define a Valence-native optional GUI helper API for inventory-backed screens, click handling, close handling, readonly slots, and lifecycle cleanup.
- Keep GUI state transitions testable through pure models where possible, with ECS systems only applying events and packets.
- Add positive and negative tests for opening, clicking, readonly slots, stale window IDs, invalid slots, close events, disconnect cleanup, and plugin-disabled behavior.
- Add examples/docs for common menus without implying vanilla container parity beyond existing inventory evidence.

## Impact

- **Files**: `valence_inventory`, optional GUI helper crate/plugin, examples/docs, tests, and Cairn artifacts.
- **Testing**: GUI model tests, packet/event integration tests, invalid-click fixtures, examples/playground smoke tests, selected inventory mc-compat dry runs, and Cairn gates/validation.
- **Non-claims**: this does not replace `valence_inventory`, does not claim all vanilla container behavior, and does not copy Hyperion GUI internals directly.
