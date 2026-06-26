# Proposal: Add inventory SystemSet contracts

## Why

`InventoryPlugin` currently registers several packet readers, state mutation systems, viewer updates, cursor updates, and packet flush preparation in tuple groups. That works, but downstream gameplay plugins must depend on implicit tuple ordering when they need to run before or after inventory input, mutation, or presentation. Named Bevy `SystemSet`s would make inventory scheduling easier to inspect and safer to compose.

## What Changes

- Inventory current `InventoryPlugin` systems, schedules, ordering constraints, events, resources, and default plugin membership.
- Define public or crate-visible inventory `SystemSet` contracts for packet input, model mutation, viewer/window synchronization, presentation/flush preparation, and cleanup if applicable.
- Move existing inventory systems into named sets without changing inventory semantics or default plugin membership.
- Add positive schedule smoke tests and negative disabled-plugin or ordering-regression tests.
- Record schedule hygiene evidence because the change intentionally affects Bevy schedule shape.

## Impact

- **Files**: `servers/valence/crates/valence_inventory/src/lib.rs`, selected inventory system modules/tests, schedule hygiene evidence under `docs/evidence/`.
- **Testing**: focused `valence_inventory` tests, schedule hygiene checker, selected example tests if ordering-sensitive behavior changes, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not change inventory protocol semantics, add new inventory behavior, alter Valence default plugin membership, or claim broader compatibility.
